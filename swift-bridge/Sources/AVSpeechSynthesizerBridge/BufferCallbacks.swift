// swiftlint:disable function_body_length
import AVFAudio
import Foundation

private func avsAudioCommonFormatName(_ format: AVAudioCommonFormat) -> String {
    switch format {
    case .otherFormat:
        return "otherFormat"
    case .pcmFormatFloat32:
        return "pcmFormatFloat32"
    case .pcmFormatFloat64:
        return "pcmFormatFloat64"
    case .pcmFormatInt16:
        return "pcmFormatInt16"
    case .pcmFormatInt32:
        return "pcmFormatInt32"
    @unknown default:
        return "unknown"
    }
}

private func avsAudioBufferPayload(
    from buffer: AVAudioPCMBuffer,
    isEndOfStream: Bool
) -> AVSAudioBufferPayload {
    let audioFileSettingsJson = try? avsJSONString(fromJSONObject: buffer.format.settings)
    var planesBase64: [String] = []

    if buffer.frameLength > 0 {
        let audioBuffers = UnsafeMutableAudioBufferListPointer(
            UnsafeMutablePointer(mutating: buffer.audioBufferList)
        )
        for audioBuffer in audioBuffers {
            guard let data = audioBuffer.mData, audioBuffer.mDataByteSize > 0 else {
                planesBase64.append(Data().base64EncodedString())
                continue
            }
            let bytes = Data(bytes: data, count: Int(audioBuffer.mDataByteSize))
            planesBase64.append(bytes.base64EncodedString())
        }
    }

    return AVSAudioBufferPayload(
        sampleRate: buffer.format.sampleRate,
        channelCount: Int(buffer.format.channelCount),
        frameLength: Int(buffer.frameLength),
        commonFormat: avsAudioCommonFormatName(buffer.format.commonFormat),
        isInterleaved: buffer.format.isInterleaved,
        planesBase64: planesBase64,
        audioFileSettingsJson: audioFileSettingsJson,
        isEndOfStream: isEndOfStream
    )
}

private func avsCollectUtteranceBuffers(
    with box: AVSSynthesizerBox,
    payload: AVSUtterancePayload
) throws -> AVSCollectedBufferWritePayload {
    let utterance = try avsUtterance(from: payload)
    let semaphore = DispatchSemaphore(value: 0)
    var capturedError: Error?
    var events: [AVSCollectedBufferWriteEvent] = []

    let bridgeBufferCallback: AVSpeechSynthesizer.BufferCallback = { buffer in
        guard capturedError == nil else { return }
        guard let pcmBuffer = buffer as? AVAudioPCMBuffer else {
            capturedError = AVSBridgeError.io(
                "AVSpeechSynthesizer emitted a non-PCM audio buffer"
            )
            semaphore.signal()
            return
        }

        let isEndOfStream = pcmBuffer.frameLength == 0
        let payload = avsAudioBufferPayload(from: pcmBuffer, isEndOfStream: isEndOfStream)
        events.append(
            AVSCollectedBufferWriteEvent(
                kind: .buffer,
                buffer: payload,
                markerBatch: nil
            )
        )
        if isEndOfStream {
            semaphore.signal()
        }
    }

    box.synthesizer.write(utterance, toBufferCallback: bridgeBufferCallback) { markers in
        events.append(
            AVSCollectedBufferWriteEvent(
                kind: .markerBatch,
                buffer: nil,
                markerBatch: AVSMarkerBatchPayload(markers: markers.map(avsMarkerPayload))
            )
        )
    }

    if !avsWaitForSignal(semaphore, timeoutSeconds: 120) {
        throw AVSBridgeError.timedOut("buffer callback synthesis timed out after 120 seconds")
    }

    if let capturedError {
        if let bridgeError = capturedError as? AVSBridgeError {
            throw bridgeError
        }
        throw AVSBridgeError.framework(capturedError)
    }

    return AVSCollectedBufferWritePayload(events: events)
}

@_cdecl("avs_synthesizer_collect_buffers_json")
public func avs_synthesizer_collect_buffers_json(
    _ token: UnsafeMutableRawPointer?,
    _ utteranceJson: UnsafePointer<CChar>?,
    _ outResultJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let box = try avsSynthesizerBox(token)
        let payload = try avsDecodeJSON(utteranceJson, as: AVSUtterancePayload.self)
        let result = try avsCollectUtteranceBuffers(with: box, payload: payload)
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
