import AVFAudio
import Foundation

private struct AVSOfflineWriteOutcome {
    var failure: Error?
    var sawCompletion: Bool
    var markers: [AVSpeechSynthesisMarker]
}

private final class AVSOfflineWrite {
    private let lock = NSLock()
    private let outputURL: URL
    private var audioFile: AVAudioFile?
    private var failure: Error?
    private var sawCompletion = false
    private var markers: [AVSpeechSynthesisMarker] = []
    private var isClosed = false

    init(outputURL: URL) {
        self.outputURL = outputURL
    }

    func handle(_ buffer: AVAudioBuffer) -> Bool {
        lock.lock()
        defer { lock.unlock() }
        guard !isClosed, failure == nil else { return false }
        guard let pcmBuffer = buffer as? AVAudioPCMBuffer else {
            failure = AVSBridgeError.io("AVSpeechSynthesizer emitted a non-PCM audio buffer")
            return true
        }
        if pcmBuffer.frameLength == 0 {
            sawCompletion = true
            return true
        }
        do {
            if audioFile == nil {
                audioFile = try AVAudioFile(
                    forWriting: outputURL,
                    settings: pcmBuffer.format.settings,
                    commonFormat: pcmBuffer.format.commonFormat,
                    interleaved: pcmBuffer.format.isInterleaved
                )
            }
            try audioFile?.write(from: pcmBuffer)
            return false
        } catch {
            failure = error
            return true
        }
    }

    func append(_ emittedMarkers: [AVSpeechSynthesisMarker]) {
        lock.lock()
        defer { lock.unlock() }
        guard !isClosed else { return }
        markers.append(contentsOf: emittedMarkers)
    }

    func close() -> AVSOfflineWriteOutcome {
        lock.lock()
        defer { lock.unlock() }
        isClosed = true
        audioFile = nil
        return AVSOfflineWriteOutcome(failure: failure, sawCompletion: sawCompletion, markers: markers)
    }
}

private func avsWriteUtterance(
    with box: AVSSynthesizerBox,
    payload: AVSUtterancePayload,
    outputPath: String
) throws -> AVSWriteResultPayload {
    let outputURL = URL(fileURLWithPath: outputPath)
    if FileManager.default.fileExists(atPath: outputPath) {
        do {
            try FileManager.default.removeItem(at: outputURL)
        } catch {
            throw AVSBridgeError.io(
                "failed to remove existing output file: \(error.localizedDescription)"
            )
        }
    }

    let utterance = try avsUtterance(from: payload)
    let semaphore = DispatchSemaphore(value: 0)
    let write = AVSOfflineWrite(outputURL: outputURL)

    let bufferCallback: AVSpeechSynthesizer.BufferCallback = { buffer in
        if write.handle(buffer) {
            semaphore.signal()
        }
    }

    box.synthesizer.write(utterance, toBufferCallback: bufferCallback) { emittedMarkers in
        write.append(emittedMarkers)
    }

    let signaled = avsWaitForSignal(semaphore, timeoutSeconds: 120)
    let result = write.close()
    if !signaled {
        throw AVSBridgeError.timedOut("offline synthesis timed out after 120 seconds")
    }
    if let capturedError = result.failure {
        if let bridgeError = capturedError as? AVSBridgeError {
            throw bridgeError
        }
        throw AVSBridgeError.framework(capturedError)
    }
    if !result.sawCompletion {
        throw AVSBridgeError.unknown(
            "offline synthesis ended without an end-of-stream buffer"
        )
    }

    return AVSWriteResultPayload(
        outputPath: outputPath,
        markers: result.markers.map(avsMarkerPayload)
    )
}

@_cdecl("avs_synthesizer_write_utterance_to_file_json")
public func avs_synthesizer_write_utterance_to_file_json(
    _ token: UnsafeMutableRawPointer?,
    _ utteranceJson: UnsafePointer<CChar>?,
    _ outputPath: UnsafePointer<CChar>?,
    _ outResultJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let box = try avsSynthesizerBox(token)
        let payload = try avsDecodeJSON(utteranceJson, as: AVSUtterancePayload.self)
        let outputPath = try avsRequireString(outputPath, field: "output path")
        let result = try avsWriteUtterance(with: box, payload: payload, outputPath: outputPath)
        outResultJson.pointee = avsCString(try avsEncodeJSON(result))
        return AVS_OK
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return AVS_UNKNOWN
    }
}
