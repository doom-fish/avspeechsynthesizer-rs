// swiftlint:disable identifier_name line_length
import AVFAudio
import Foundation

// Event kind constants for the async stream
let AVS_EVENT_DID_START: Int32 = 0
let AVS_EVENT_DID_FINISH: Int32 = 1
let AVS_EVENT_DID_PAUSE: Int32 = 2
let AVS_EVENT_DID_CONTINUE: Int32 = 3
let AVS_EVENT_DID_CANCEL: Int32 = 4
let AVS_EVENT_WILL_SPEAK_RANGE: Int32 = 5
let AVS_EVENT_WILL_SPEAK_MARKER: Int32 = 6

// Callback type for async stream events
public typealias AVSAsyncStreamCallback = @convention(c) (Int32, UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void

// Event payload for async stream
struct AVSSynthesisEventPayload: Codable {
    var kind: Int32
    var utterance: AVSUtterancePayload
    var characterRange: AVSRangePayload?
    var marker: AVSMarkerPayload?
}

/// Bridge class that conforms to AVSpeechSynthesizerDelegate and forwards events to a C callback
final class AVSSynthesisEventBridge: NSObject, AVSpeechSynthesizerDelegate {
    let onEvent: AVSAsyncStreamCallback
    let onClose: AVSContextCallback?
    let retention: AVSContextRetention
    weak var hub: AVSDelegateHub?

    init(
        onEvent: @escaping AVSAsyncStreamCallback,
        onClose: AVSContextCallback?,
        ctx: UnsafeMutableRawPointer?,
        retain: AVSContextCallback?,
        release: AVSContextCallback?,
        hub: AVSDelegateHub
    ) {
        self.onEvent = onEvent
        self.onClose = onClose
        self.retention = AVSContextRetention(context: ctx, retain: retain, release: release)
        self.hub = hub
        super.init()
    }

    func close() {
        onClose?(retention.context)
    }

    private func emit(kind: Int32, utterance: AVSpeechUtterance, characterRange: NSRange? = nil, marker: AVSpeechSynthesisMarker? = nil) {
        let payload = AVSSynthesisEventPayload(
            kind: kind,
            utterance: avsUtterancePayload(from: utterance),
            characterRange: characterRange.map(avsRangePayload(from:)),
            marker: marker.map(avsMarkerPayload(from:))
        )

        do {
            let json = try avsEncodeJSON(payload)
            // The pointer provided by `withCString` is only valid for the
            // duration of the closure. Invoke the C callback inside the closure
            // so Rust never reads a dangling/freed string pointer.
            json.withCString { cStr in
                onEvent(kind, UnsafeMutableRawPointer(mutating: cStr), retention.context)
            }
        } catch {
            // Silently drop on encoding error
            onEvent(kind, nil, retention.context)
        }
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didStart utterance: AVSpeechUtterance) {
        emit(kind: AVS_EVENT_DID_START, utterance: utterance)
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didFinish utterance: AVSpeechUtterance) {
        emit(kind: AVS_EVENT_DID_FINISH, utterance: utterance)
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didPause utterance: AVSpeechUtterance) {
        emit(kind: AVS_EVENT_DID_PAUSE, utterance: utterance)
    }

    func speechSynthesizer(
        _ synthesizer: AVSpeechSynthesizer,
        didContinue utterance: AVSpeechUtterance
    ) {
        emit(kind: AVS_EVENT_DID_CONTINUE, utterance: utterance)
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didCancel utterance: AVSpeechUtterance) {
        emit(kind: AVS_EVENT_DID_CANCEL, utterance: utterance)
    }

    func speechSynthesizer(
        _ synthesizer: AVSpeechSynthesizer,
        willSpeakRangeOfSpeechString characterRange: NSRange,
        utterance: AVSpeechUtterance
    ) {
        emit(kind: AVS_EVENT_WILL_SPEAK_RANGE, utterance: utterance, characterRange: characterRange)
    }

    @available(macOS 14.0, *)
    func speechSynthesizer(
        _ synthesizer: AVSpeechSynthesizer,
        willSpeak marker: AVSpeechSynthesisMarker,
        utterance: AVSpeechUtterance
    ) {
        emit(kind: AVS_EVENT_WILL_SPEAK_MARKER, utterance: utterance, marker: marker)
    }
}

extension AVSSynthesisEventBridge: @unchecked Sendable {}

// C-side FFI functions for subscribing to async events

@_cdecl("avs_synthesis_event_subscribe")
public func avs_synthesis_event_subscribe(
    _ token: UnsafeMutableRawPointer?,
    _ onEvent: @escaping AVSAsyncStreamCallback,
    _ onClose: AVSContextCallback?,
    _ ctx: UnsafeMutableRawPointer?,
    _ ctxRetain: AVSContextCallback?,
    _ ctxRelease: AVSContextCallback?
) -> UnsafeMutableRawPointer? {
    guard let token else {
        return nil
    }
    let box: AVSSynthesizerBox = avsBorrow(token)
    let bridge = AVSSynthesisEventBridge(
        onEvent: onEvent,
        onClose: onClose,
        ctx: ctx,
        retain: ctxRetain,
        release: ctxRelease,
        hub: box.hub
    )
    box.hub.add(bridge)
    return avsRetain(bridge)
}

@_cdecl("avs_synthesis_event_unsubscribe")
public func avs_synthesis_event_unsubscribe(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    let bridge = Unmanaged<AVSSynthesisEventBridge>.fromOpaque(handle).takeRetainedValue()
    bridge.hub?.remove(bridge)
}

@_cdecl("avs_synthesizer_deliver_test_start")
public func avs_synthesizer_deliver_test_start(
    _ token: UnsafeMutableRawPointer?,
    _ text: UnsafePointer<CChar>?
) {
    guard let token, let text else { return }
    let box: AVSSynthesizerBox = avsBorrow(token)
    box.hub.speechSynthesizer(box.synthesizer, didStart: AVSpeechUtterance(string: String(cString: text)))
}

