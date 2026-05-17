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
    let ctx: UnsafeMutableRawPointer?

    init(onEvent: @escaping AVSAsyncStreamCallback, ctx: UnsafeMutableRawPointer?) {
        self.onEvent = onEvent
        self.ctx = ctx
        super.init()
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
            var payloadPtr: UnsafeMutableRawPointer?
            json.withCString { cStr in
                payloadPtr = UnsafeMutableRawPointer(mutating: cStr)
            }
            onEvent(kind, payloadPtr, ctx)
        } catch {
            // Silently drop on encoding error
            onEvent(kind, nil, ctx)
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
    _ ctx: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let token else {
        return nil
    }
    let box: AVSSynthesizerBox = avsBorrow(token)
    let bridge = AVSSynthesisEventBridge(onEvent: onEvent, ctx: ctx)
    box.synthesizer.delegate = bridge
    return avsRetain(bridge)
}

@_cdecl("avs_synthesis_event_unsubscribe")
public func avs_synthesis_event_unsubscribe(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    avsRelease(handle)
}

