import AVFAudio
import Foundation

let AVS_OK: Int32 = 0
let AVS_INVALID_ARGUMENT: Int32 = -1
let AVS_UNAVAILABLE_ON_THIS_MACOS: Int32 = -2
let AVS_TIMED_OUT: Int32 = -3
let AVS_IO_ERROR: Int32 = -4
let AVS_FRAMEWORK_ERROR: Int32 = -5
let AVS_UNKNOWN: Int32 = -99

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

public func avs_block_on_async(
    timeoutSeconds: Int = 30,
    work: @escaping () async throws -> Void
) -> Int32 {
    let semaphore = DispatchSemaphore(value: 0)
    var status = AVS_OK
    Task {
        do {
            try await work()
        } catch {
            status = AVS_FRAMEWORK_ERROR
        }
        semaphore.signal()
    }
    if semaphore.wait(timeout: .now() + .seconds(timeoutSeconds)) == .timedOut {
        return AVS_TIMED_OUT
    }
    return status
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

func avsNormalizedLanguage(_ language: String) -> String {
    language.replacingOccurrences(of: "_", with: "-").lowercased()
}

struct AVSVoicePayload: Codable {
    var language: String
    var identifier: String
    var name: String
    var quality: Int
    var gender: Int?
}

struct AVSUtterancePayload: Codable {
    var speechString: String
    var voice: AVSVoicePayload?
    var rate: Float
    var pitchMultiplier: Float
    var volume: Float
    var preUtteranceDelay: Double
    var postUtteranceDelay: Double
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

func avsVoicePayload(from voice: AVSpeechSynthesisVoice) -> AVSVoicePayload {
    let gender: Int?
    if #available(macOS 10.15, *) {
        gender = Int(voice.gender.rawValue)
    } else {
        gender = nil
    }
    return AVSVoicePayload(
        language: voice.language,
        identifier: voice.identifier,
        name: voice.name,
        quality: Int(voice.quality.rawValue),
        gender: gender
    )
}

func avsVoice(from payload: AVSVoicePayload?) -> AVSpeechSynthesisVoice? {
    guard let payload else { return nil }
    if !payload.identifier.isEmpty,
       let byIdentifier = AVSpeechSynthesisVoice(identifier: payload.identifier)
    {
        return byIdentifier
    }
    return AVSpeechSynthesisVoice(language: payload.language)
}

func avsUtterance(from payload: AVSUtterancePayload) -> AVSpeechUtterance {
    let utterance = AVSpeechUtterance(string: payload.speechString)
    utterance.voice = avsVoice(from: payload.voice)
    utterance.rate = payload.rate
    utterance.pitchMultiplier = payload.pitchMultiplier
    utterance.volume = payload.volume
    utterance.preUtteranceDelay = payload.preUtteranceDelay
    utterance.postUtteranceDelay = payload.postUtteranceDelay
    return utterance
}

func avsUtterancePayload(from utterance: AVSpeechUtterance) -> AVSUtterancePayload {
    AVSUtterancePayload(
        speechString: utterance.speechString,
        voice: utterance.voice.map(avsVoicePayload),
        rate: utterance.rate,
        pitchMultiplier: utterance.pitchMultiplier,
        volume: utterance.volume,
        preUtteranceDelay: utterance.preUtteranceDelay,
        postUtteranceDelay: utterance.postUtteranceDelay
    )
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
