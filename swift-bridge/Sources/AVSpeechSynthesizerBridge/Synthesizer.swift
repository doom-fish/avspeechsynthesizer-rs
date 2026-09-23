import AVFAudio
import Foundation

final class AVSRustSpeechDelegate: NSObject, AVSpeechSynthesizerDelegate {
    let callback: AVSJSONCallback
    let retention: AVSContextRetention

    init(
        callback: @escaping AVSJSONCallback,
        userInfo: UnsafeMutableRawPointer?,
        retain: AVSContextCallback?,
        release: AVSContextCallback?
    ) {
        self.callback = callback
        self.retention = AVSContextRetention(context: userInfo, retain: retain, release: release)
        super.init()
    }

    private func emit(_ payload: AVSEventPayload) {
        avsEmitJSON(callback, userInfo: retention.context, payload: payload)
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

extension AVSRustSpeechDelegate: @unchecked Sendable {}

final class AVSDelegateHub: NSObject, AVSpeechSynthesizerDelegate {
    private let lock = NSLock()
    private var handler: AVSRustSpeechDelegate?
    private var subscribers: [AVSSynthesisEventBridge] = []

    func setHandler(_ newHandler: AVSRustSpeechDelegate?) {
        lock.lock()
        let previous = handler
        handler = newHandler
        lock.unlock()
        withExtendedLifetime(previous) {}
    }

    func add(_ subscriber: AVSSynthesisEventBridge) {
        lock.lock()
        subscribers.append(subscriber)
        lock.unlock()
    }

    func remove(_ subscriber: AVSSynthesisEventBridge) {
        lock.lock()
        let removed = subscribers.firstIndex { $0 === subscriber }.map { subscribers.remove(at: $0) }
        lock.unlock()
        withExtendedLifetime(removed) {}
    }

    var subscriberCount: Int {
        lock.lock()
        defer { lock.unlock() }
        return subscribers.count
    }

    var hasHandler: Bool {
        lock.lock()
        defer { lock.unlock() }
        return handler != nil
    }

    private func listeners() -> [AVSpeechSynthesizerDelegate] {
        lock.lock()
        defer { lock.unlock() }
        var listeners: [AVSpeechSynthesizerDelegate] = []
        if let handler {
            listeners.append(handler)
        }
        listeners.append(contentsOf: subscribers)
        return listeners
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didStart utterance: AVSpeechUtterance) {
        for listener in listeners() {
            listener.speechSynthesizer?(synthesizer, didStart: utterance)
        }
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didFinish utterance: AVSpeechUtterance) {
        for listener in listeners() {
            listener.speechSynthesizer?(synthesizer, didFinish: utterance)
        }
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didPause utterance: AVSpeechUtterance) {
        for listener in listeners() {
            listener.speechSynthesizer?(synthesizer, didPause: utterance)
        }
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didContinue utterance: AVSpeechUtterance) {
        for listener in listeners() {
            listener.speechSynthesizer?(synthesizer, didContinue: utterance)
        }
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didCancel utterance: AVSpeechUtterance) {
        for listener in listeners() {
            listener.speechSynthesizer?(synthesizer, didCancel: utterance)
        }
    }

    func speechSynthesizer(
        _ synthesizer: AVSpeechSynthesizer,
        willSpeakRangeOfSpeechString characterRange: NSRange,
        utterance: AVSpeechUtterance
    ) {
        for listener in listeners() {
            listener.speechSynthesizer?(
                synthesizer,
                willSpeakRangeOfSpeechString: characterRange,
                utterance: utterance
            )
        }
    }

    @available(macOS 14.0, *)
    func speechSynthesizer(
        _ synthesizer: AVSpeechSynthesizer,
        willSpeak marker: AVSpeechSynthesisMarker,
        utterance: AVSpeechUtterance
    ) {
        for listener in listeners() {
            listener.speechSynthesizer?(synthesizer, willSpeak: marker, utterance: utterance)
        }
    }
}

extension AVSDelegateHub: @unchecked Sendable {}

final class AVSSynthesizerBox: NSObject {
    let synthesizer = AVSpeechSynthesizer()
    let hub = AVSDelegateHub()

    override init() {
        super.init()
        synthesizer.delegate = hub
    }

    func setEventHandler(
        callback: AVSJSONCallback?,
        userInfo: UnsafeMutableRawPointer?,
        retain: AVSContextCallback?,
        release: AVSContextCallback?
    ) {
        hub.setHandler(callback.map {
            AVSRustSpeechDelegate(callback: $0, userInfo: userInfo, retain: retain, release: release)
        })
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
    _ callback: AVSJSONCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ retain: AVSContextCallback?,
    _ release: AVSContextCallback?
) {
    guard let token else { return }
    let box: AVSSynthesizerBox = avsBorrow(token)
    box.setEventHandler(callback: callback, userInfo: userInfo, retain: retain, release: release)
}

@_cdecl("avs_synthesizer_listener_counts")
public func avs_synthesizer_listener_counts(
    _ token: UnsafeMutableRawPointer?,
    _ outHasHandler: UnsafeMutablePointer<Bool>?,
    _ outSubscribers: UnsafeMutablePointer<Int>?
) {
    guard let token else { return }
    let box: AVSSynthesizerBox = avsBorrow(token)
    outHasHandler?.pointee = box.hub.hasHandler && box.synthesizer.delegate === box.hub
    outSubscribers?.pointee = box.hub.subscriberCount
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
        box.synthesizer.speak(try avsUtterance(from: payload))
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

@_cdecl("avs_available_voices_did_change_notification_name")
public func avs_available_voices_did_change_notification_name() -> UnsafeMutablePointer<CChar>? {
    if #available(macOS 14.0, *) {
        return avsCString(AVSpeechSynthesizer.availableVoicesDidChangeNotification.rawValue)
    }
    return avsCString("AVSpeechSynthesisAvailableVoicesDidChangeNotification")
}

@_cdecl("avs_run_loop_pump")
public func avs_run_loop_pump(_ seconds: Double) {
    let seconds = max(0, seconds)
    RunLoop.current.run(until: Date(timeIntervalSinceNow: seconds))
}
