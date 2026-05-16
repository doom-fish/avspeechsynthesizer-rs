import AVFAudio
import Foundation

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

    let utterance = avsUtterance(from: payload)
    let semaphore = DispatchSemaphore(value: 0)
    var audioFile: AVAudioFile?
    var capturedError: Error?
    var sawCompletion = false
    var markers: [AVSpeechSynthesisMarker] = []

    let bufferCallback: AVSpeechSynthesizer.BufferCallback = { buffer in
        guard capturedError == nil else { return }
        guard let pcmBuffer = buffer as? AVAudioPCMBuffer else {
            capturedError = AVSBridgeError.io(
                "AVSpeechSynthesizer emitted a non-PCM audio buffer"
            )
            semaphore.signal()
            return
        }
        if pcmBuffer.frameLength == 0 {
            sawCompletion = true
            semaphore.signal()
            return
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
        } catch {
            capturedError = error
            semaphore.signal()
        }
    }

    if #available(macOS 13.0, *) {
        box.synthesizer.write(utterance, toBufferCallback: bufferCallback) { emittedMarkers in
            markers.append(contentsOf: emittedMarkers)
        }
    } else {
        box.synthesizer.write(utterance, toBufferCallback: bufferCallback)
    }

    if semaphore.wait(timeout: .now() + .seconds(120)) == .timedOut {
        throw AVSBridgeError.timedOut("offline synthesis timed out after 120 seconds")
    }
    if let capturedError {
        if let bridgeError = capturedError as? AVSBridgeError {
            throw bridgeError
        }
        throw AVSBridgeError.framework(capturedError)
    }
    if !sawCompletion {
        throw AVSBridgeError.unknown(
            "offline synthesis ended without an end-of-stream buffer"
        )
    }

    return AVSWriteResultPayload(
        outputPath: outputPath,
        markers: markers.map(avsMarkerPayload)
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
