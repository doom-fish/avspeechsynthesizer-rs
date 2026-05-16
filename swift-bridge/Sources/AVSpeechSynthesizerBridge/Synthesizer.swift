import AVFAudio
import Foundation

public typealias AVSEventCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void

final class AVSRustSpeechDelegate: NSObject, AVSpeechSynthesizerDelegate {
    let callback: AVSEventCallback
    let userInfo: UnsafeMutableRawPointer?

    init(callback: @escaping AVSEventCallback, userInfo: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.userInfo = userInfo
        super.init()
    }

    private func emit(_ payload: AVSEventPayload) {
        guard let json = try? avsEncodeJSON(payload) else {
            callback(userInfo, nil)
            return
        }
        json.withCString { callback(userInfo, $0) }
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didStart utterance: AVSpeechUtterance) {
        emit(AVSEventPayload(
            event: "didStart",
            utterance: avsUtterancePayload(from: utterance),
            characterRange: nil,
            marker: nil
        ))
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didFinish utterance: AVSpeechUtterance) {
        emit(AVSEventPayload(
            event: "didFinish",
            utterance: avsUtterancePayload(from: utterance),
            characterRange: nil,
            marker: nil
        ))
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didPause utterance: AVSpeechUtterance) {
        emit(AVSEventPayload(
            event: "didPause",
            utterance: avsUtterancePayload(from: utterance),
            characterRange: nil,
            marker: nil
        ))
    }

    func speechSynthesizer(
        _ synthesizer: AVSpeechSynthesizer,
        didContinue utterance: AVSpeechUtterance
    ) {
        emit(AVSEventPayload(
            event: "didContinue",
            utterance: avsUtterancePayload(from: utterance),
            characterRange: nil,
            marker: nil
        ))
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didCancel utterance: AVSpeechUtterance) {
        emit(AVSEventPayload(
            event: "didCancel",
            utterance: avsUtterancePayload(from: utterance),
            characterRange: nil,
            marker: nil
        ))
    }

    func speechSynthesizer(
        _ synthesizer: AVSpeechSynthesizer,
        willSpeakRangeOfSpeechString characterRange: NSRange,
        utterance: AVSpeechUtterance
    ) {
        emit(AVSEventPayload(
            event: "willSpeakRangeOfSpeechString",
            utterance: avsUtterancePayload(from: utterance),
            characterRange: avsRangePayload(from: characterRange),
            marker: nil
        ))
    }

    @available(macOS 14.0, *)
    func speechSynthesizer(
        _ synthesizer: AVSpeechSynthesizer,
        willSpeak marker: AVSpeechSynthesisMarker,
        utterance: AVSpeechUtterance
    ) {
        emit(AVSEventPayload(
            event: "willSpeakMarker",
            utterance: avsUtterancePayload(from: utterance),
            characterRange: nil,
            marker: avsMarkerPayload(from: marker)
        ))
    }
}

final class AVSSynthesizerBox: NSObject {
    let synthesizer = AVSpeechSynthesizer()
    var delegateBox: AVSRustSpeechDelegate?

    func setEventHandler(callback: AVSEventCallback?, userInfo: UnsafeMutableRawPointer?) {
        guard let callback else {
            synthesizer.delegate = nil
            delegateBox = nil
            return
        }
        let delegateBox = AVSRustSpeechDelegate(callback: callback, userInfo: userInfo)
        self.delegateBox = delegateBox
        synthesizer.delegate = delegateBox
    }
}

func avsSynthesizerBox(_ token: UnsafeMutableRawPointer?) throws -> AVSSynthesizerBox {
    guard let token else {
        throw AVSBridgeError.invalidArgument("missing synthesizer token")
    }
    return avsBorrow(token)
}

func avsBoundary(from rawValue: Int32) -> AVSpeechBoundary {
    rawValue == 1 ? .word : .immediate
}

@_cdecl("avs_synthesizer_new")
public func avs_synthesizer_new() -> UnsafeMutableRawPointer {
    avsRetain(AVSSynthesizerBox())
}

@_cdecl("avs_synthesizer_release")
public func avs_synthesizer_release(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    avsRelease(token)
}

@_cdecl("avs_synthesizer_set_event_handler")
public func avs_synthesizer_set_event_handler(
    _ token: UnsafeMutableRawPointer?,
    _ callback: AVSEventCallback?,
    _ userInfo: UnsafeMutableRawPointer?
) {
    guard let token else { return }
    let box: AVSSynthesizerBox = avsBorrow(token)
    box.setEventHandler(callback: callback, userInfo: userInfo)
}

@_cdecl("avs_synthesizer_is_speaking")
public func avs_synthesizer_is_speaking(_ token: UnsafeMutableRawPointer?) -> Bool {
    guard let token else { return false }
    let box: AVSSynthesizerBox = avsBorrow(token)
    return box.synthesizer.isSpeaking
}

@_cdecl("avs_synthesizer_is_paused")
public func avs_synthesizer_is_paused(_ token: UnsafeMutableRawPointer?) -> Bool {
    guard let token else { return false }
    let box: AVSSynthesizerBox = avsBorrow(token)
    return box.synthesizer.isPaused
}

@_cdecl("avs_synthesizer_speak_json")
public func avs_synthesizer_speak_json(
    _ token: UnsafeMutableRawPointer?,
    _ utteranceJson: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let box = try avsSynthesizerBox(token)
        let payload = try avsDecodeJSON(utteranceJson, as: AVSUtterancePayload.self)
        box.synthesizer.speak(avsUtterance(from: payload))
        return AVS_OK
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return AVS_UNKNOWN
    }
}

@_cdecl("avs_synthesizer_pause")
public func avs_synthesizer_pause(_ token: UnsafeMutableRawPointer?, _ boundary: Int32) -> Bool {
    guard let token else { return false }
    let box: AVSSynthesizerBox = avsBorrow(token)
    return box.synthesizer.pauseSpeaking(at: avsBoundary(from: boundary))
}

@_cdecl("avs_synthesizer_stop")
public func avs_synthesizer_stop(_ token: UnsafeMutableRawPointer?, _ boundary: Int32) -> Bool {
    guard let token else { return false }
    let box: AVSSynthesizerBox = avsBorrow(token)
    return box.synthesizer.stopSpeaking(at: avsBoundary(from: boundary))
}

@_cdecl("avs_synthesizer_continue")
public func avs_synthesizer_continue(_ token: UnsafeMutableRawPointer?) -> Bool {
    guard let token else { return false }
    let box: AVSSynthesizerBox = avsBorrow(token)
    return box.synthesizer.continueSpeaking()
}

@_cdecl("avs_run_loop_pump")
public func avs_run_loop_pump(_ seconds: Double) {
    let seconds = max(0, seconds)
    RunLoop.current.run(until: Date(timeIntervalSinceNow: seconds))
}
