// swiftlint:disable identifier_name
import AVFAudio
import Foundation

let AVS_OK: Int32 = 0
let AVS_INVALID_ARGUMENT: Int32 = -1
let AVS_UNAVAILABLE_ON_THIS_MACOS: Int32 = -2
let AVS_TIMED_OUT: Int32 = -3
let AVS_IO_ERROR: Int32 = -4
let AVS_FRAMEWORK_ERROR: Int32 = -5
let AVS_UNKNOWN: Int32 = -99

public typealias AVSJSONCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void

@_cdecl("avs_string_free")
public func avs_string_free(_ string: UnsafeMutablePointer<CChar>?) {
    guard let string else { return }
    free(string)
}

@inline(__always)
func avsCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
func avsRetain(_ object: some AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func avsBorrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as type: T.Type = T.self) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@inline(__always)
func avsRelease(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

enum AVSBridgeError: Error, CustomStringConvertible {
    case invalidArgument(String)
    case unavailableOnThisMacOS(String)
    case timedOut(String)
    case io(String)
    case framework(Error)
    case unknown(String)

    var description: String {
        switch self {
        case let .invalidArgument(message),
            let .unavailableOnThisMacOS(message),
            let .timedOut(message),
            let .io(message),
            let .unknown(message):
            return message
        case let .framework(error):
            return error.localizedDescription
        }
    }

    var statusCode: Int32 {
        switch self {
        case .invalidArgument:
            return AVS_INVALID_ARGUMENT
        case .unavailableOnThisMacOS:
            return AVS_UNAVAILABLE_ON_THIS_MACOS
        case .timedOut:
            return AVS_TIMED_OUT
        case .io:
            return AVS_IO_ERROR
        case .framework:
            return AVS_FRAMEWORK_ERROR
        case .unknown:
            return AVS_UNKNOWN
        }
    }
}

func avsEncodeJSON<T: Encodable>(_ value: T) throws -> String {
    let data = try JSONEncoder().encode(value)
    guard let string = String(data: data, encoding: .utf8) else {
        throw AVSBridgeError.unknown("failed to encode JSON as UTF-8")
    }
    return string
}

func avsDecodeJSON<T: Decodable>(_ cString: UnsafePointer<CChar>?, as type: T.Type) throws -> T {
    guard let cString else {
        throw AVSBridgeError.invalidArgument("missing JSON payload")
    }
    let data = Data(String(cString: cString).utf8)
    do {
        return try JSONDecoder().decode(T.self, from: data)
    } catch {
        throw AVSBridgeError.invalidArgument("invalid JSON payload: \(error.localizedDescription)")
    }
}

func avsRequireString(_ cString: UnsafePointer<CChar>?, field: String) throws -> String {
    guard let cString else {
        throw AVSBridgeError.invalidArgument("missing \(field)")
    }
    return String(cString: cString)
}

func avsJSONString(fromJSONObject object: Any) throws -> String {
    guard JSONSerialization.isValidJSONObject(object) else {
        throw AVSBridgeError.invalidArgument("object is not JSON serializable")
    }
    let data = try JSONSerialization.data(withJSONObject: object, options: [.sortedKeys])
    guard let string = String(data: data, encoding: .utf8) else {
        throw AVSBridgeError.unknown("failed to encode Foundation object as UTF-8 JSON")
    }
    return string
}

func avsJSONObject(from json: String) throws -> Any {
    let data = Data(json.utf8)
    return try JSONSerialization.jsonObject(with: data)
}

func avsJSONObjectDictionary(from json: String, field: String) throws -> [String: Any] {
    guard let dictionary = try avsJSONObject(from: json) as? [String: Any] else {
        throw AVSBridgeError.invalidArgument("\(field) JSON must decode to an object")
    }
    return dictionary
}

func avsEmitJSON<T: Encodable>(
    _ callback: AVSJSONCallback?,
    userInfo: UnsafeMutableRawPointer?,
    payload: T
) {
    guard let callback else { return }
    guard let json = try? avsEncodeJSON(payload) else {
        callback(userInfo, nil)
        return
    }
    json.withCString { callback(userInfo, $0) }
}

func avsWaitForSignal(_ semaphore: DispatchSemaphore, timeoutSeconds: TimeInterval) -> Bool {
    let deadline = Date().addingTimeInterval(timeoutSeconds)
    while Date() < deadline {
        if semaphore.wait(timeout: .now()) == .success {
            return true
        }
        RunLoop.current.run(mode: .default, before: Date(timeIntervalSinceNow: 0.01))
    }
    return false
}

func avsNormalizedLanguage(_ language: String) -> String {
    language.replacingOccurrences(of: "_", with: "-").lowercased()
}

struct AVSVoicePayload: Codable {
    var language: String
    var identifier: String
    var name: String
    var quality: Int
    var gender: Int?
    var audioFileSettingsJson: String?
    var voiceTraits: UInt64?
}

func avsVoicePayload(from voice: AVSpeechSynthesisVoice) -> AVSVoicePayload {
    let gender: Int?
    if #available(macOS 10.15, *) {
        gender = Int(voice.gender.rawValue)
    } else {
        gender = nil
    }

    let voiceTraits: UInt64?
    if #available(macOS 14.0, *) {
        voiceTraits = UInt64(voice.voiceTraits.rawValue)
    } else {
        voiceTraits = nil
    }

    let audioFileSettingsJson = try? avsJSONString(fromJSONObject: voice.audioFileSettings)

    return AVSVoicePayload(
        language: voice.language,
        identifier: voice.identifier,
        name: voice.name,
        quality: Int(voice.quality.rawValue),
        gender: gender,
        audioFileSettingsJson: audioFileSettingsJson,
        voiceTraits: voiceTraits
    )
}

func avsVoice(from payload: AVSVoicePayload?) -> AVSpeechSynthesisVoice? {
    guard let payload else { return nil }
    if !payload.identifier.isEmpty,
       let byIdentifier = AVSpeechSynthesisVoice(identifier: payload.identifier)
    {
        return byIdentifier
    }
    if payload.language.isEmpty {
        return AVSpeechSynthesisVoice(language: nil)
    }
    return AVSpeechSynthesisVoice(language: payload.language)
}

struct AVSRangePayload: Codable {
    var location: Int
    var length: Int
}

struct AVSMarkerPayload: Codable {
    var mark: Int
    var byteSampleOffset: UInt64
    var textRange: AVSRangePayload
    var bookmarkName: String?
    var phoneme: String?
}

func avsRangePayload(from range: NSRange) -> AVSRangePayload {
    AVSRangePayload(location: range.location, length: range.length)
}

func avsMarkerPayload(from marker: AVSpeechSynthesisMarker) -> AVSMarkerPayload {
    let bookmarkName: String?
    let phoneme: String?
    if #available(macOS 14.0, *) {
        bookmarkName = marker.bookmarkName
        phoneme = marker.phoneme
    } else {
        bookmarkName = nil
        phoneme = nil
    }
    return AVSMarkerPayload(
        mark: Int(marker.mark.rawValue),
        byteSampleOffset: UInt64(marker.byteSampleOffset),
        textRange: avsRangePayload(from: marker.textRange),
        bookmarkName: bookmarkName,
        phoneme: phoneme
    )
}

struct AVSEventPayload: Codable {
    var event: String
    var utterance: AVSUtterancePayload
    var characterRange: AVSRangePayload?
    var marker: AVSMarkerPayload?
}

struct AVSWriteResultPayload: Codable {
    var outputPath: String
    var markers: [AVSMarkerPayload]
}

struct AVSAudioBufferPayload: Codable {
    var sampleRate: Double
    var channelCount: Int
    var frameLength: Int
    var commonFormat: String
    var isInterleaved: Bool
    var planesBase64: [String]
    var audioFileSettingsJson: String?
    var isEndOfStream: Bool
}

struct AVSMarkerBatchPayload: Codable {
    var markers: [AVSMarkerPayload]
}

enum AVSCollectedBufferWriteEventKind: String, Codable {
    case buffer
    case markerBatch
}

struct AVSCollectedBufferWriteEvent: Codable {
    var kind: AVSCollectedBufferWriteEventKind
    var buffer: AVSAudioBufferPayload?
    var markerBatch: AVSMarkerBatchPayload?
}

struct AVSCollectedBufferWritePayload: Codable {
    var events: [AVSCollectedBufferWriteEvent]
}

struct AVSProviderVoicePayload: Codable {
    var name: String
    var identifier: String
    var primaryLanguages: [String]
    var supportedLanguages: [String]
    var voiceSize: Int64
    var version: String
    var gender: Int
    var age: Int
}
