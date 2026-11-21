import AVFAudio
import Foundation

public typealias AVSPcmCallback = @convention(c) (
    UnsafeMutableRawPointer?, Double, UInt32, UInt32, UInt, Bool, UnsafeRawPointer?, UnsafePointer<CChar>?
) -> Bool
public typealias AVSMarkersCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Bool

private final class AVSBufferDelivery {
    let finished = DispatchSemaphore(value: 0)
    private let lock = NSLock()
    private let dispatch: () -> Bool
    private var pending = 0
    private var draining = false
    private var closed = false
    private var failure: Error?

    init(dispatch: @escaping () -> Bool) {
        self.dispatch = dispatch
    }

    func deliver(last: Bool = false, _ enqueue: () throws -> Bool) {
        lock.lock()
        guard !closed else { lock.unlock(); return }
        do {
            guard pending < 64 else { throw AVSBridgeError.io("speech callback queue is full") }
            guard try enqueue() else { throw AVSBridgeError.io("speech buffer or marker conversion failed") }
            pending += 1
            closed = last
        } catch {
            failure = error
            closed = true
        }
        guard !draining else { lock.unlock(); return }
        draining = true
        while pending > 0 && failure == nil {
            pending -= 1
            lock.unlock()
            let ok = dispatch()
            lock.lock()
            if !ok {
                failure = AVSBridgeError.io("speech callback failed")
                closed = true
            }
        }
        draining = false
        let done = closed
        lock.unlock()
        if done { finished.signal() }
    }

    func finish() throws {
        let signaled = avsWaitForSignal(finished, timeoutSeconds: 120)
        lock.lock()
        defer { lock.unlock() }
        if !signaled {
            closed = true
            if failure == nil { failure = AVSBridgeError.timedOut("speech buffer synthesis timed out") }
        }
        if let failure { throw failure }
    }
}

@_cdecl("avs_synthesizer_stream_buffers")
public func avs_synthesizer_stream_buffers(
    _ token: UnsafeMutableRawPointer?,
    _ utteranceJson: UnsafePointer<CChar>?,
    _ context: UnsafeMutableRawPointer?,
    _ retain: AVSContextCallback?,
    _ release: AVSContextCallback?,
    _ bufferCallback: AVSPcmCallback?,
    _ markerCallback: AVSMarkersCallback?,
    _ dispatch: (@convention(c) (UnsafeMutableRawPointer?) -> Bool)?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let bufferCallback, let context, let retain, let release, let dispatch else {
            throw AVSBridgeError.invalidArgument("missing speech callback or context")
        }
        let box = try avsSynthesizerBox(token)
        let utterance = try avsUtterance(from: avsDecodeJSON(utteranceJson, as: AVSUtterancePayload.self))
        let owner = AVSContextRetention(context: context, retain: retain, release: release)
        let delivery = AVSBufferDelivery { dispatch(owner.context) }
        box.synthesizer.write(utterance, toBufferCallback: { buffer in
            let pcm = buffer as? AVAudioPCMBuffer
            delivery.deliver(last: pcm?.frameLength == 0) {
                guard let pcm else { throw AVSBridgeError.io("speech synthesizer returned a non-PCM buffer") }
                let format = pcm.format
                func send(_ settings: UnsafePointer<CChar>?) -> Bool {
                    bufferCallback(
                        owner.context, format.sampleRate, format.channelCount, pcm.frameLength,
                        format.commonFormat.rawValue, format.isInterleaved, pcm.audioBufferList, settings
                    )
                }
                if JSONSerialization.isValidJSONObject(format.settings) {
                    return try avsJSONString(fromJSONObject: format.settings).withCString(send)
                }
                return send(nil)
            }
        }, toMarkerCallback: { markers in
            guard let markerCallback else { return }
            delivery.deliver {
                try avsEncodeJSON(markers.map(avsMarkerPayload)).withCString { markerCallback(owner.context, $0) }
            }
        })
        try delivery.finish()
        return AVS_OK
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return AVS_UNKNOWN
    }
}
