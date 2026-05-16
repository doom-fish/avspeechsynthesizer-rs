// swiftlint:disable function_body_length cyclomatic_complexity
import AVFAudio
import Foundation

enum AVSMarkerConstructor: String, Codable {
    case generic
    case word
    case sentence
    case paragraph
    case phoneme
    case bookmark
}

struct AVSMarkerConstructorPayload: Codable {
    var constructor: AVSMarkerConstructor
    var mark: Int
    var byteSampleOffset: UInt64
    var textRange: AVSRangePayload
    var bookmarkName: String?
    var phoneme: String?
}

private func avsMarkerMark(from rawValue: Int) throws -> AVSpeechSynthesisMarker.Mark {
    guard let mark = AVSpeechSynthesisMarker.Mark(rawValue: rawValue) else {
        throw AVSBridgeError.invalidArgument("unknown speech synthesis marker mark: \(rawValue)")
    }
    return mark
}

private func avsNSRange(from payload: AVSRangePayload) -> NSRange {
    NSRange(location: payload.location, length: payload.length)
}

private func avsByteOffset(_ value: UInt64) throws -> Int {
    guard let byteOffset = Int(exactly: value) else {
        throw AVSBridgeError.invalidArgument("marker byte sample offset does not fit in Int")
    }
    return byteOffset
}

private func avsMarker(from payload: AVSMarkerConstructorPayload) throws -> AVSpeechSynthesisMarker {
    let byteOffset = try avsByteOffset(payload.byteSampleOffset)
    switch payload.constructor {
    case .generic:
        return AVSpeechSynthesisMarker(
            markerType: try avsMarkerMark(from: payload.mark),
            forTextRange: avsNSRange(from: payload.textRange),
            atByteSampleOffset: byteOffset
        )
    case .word:
        guard #available(macOS 14.0, *) else {
            throw AVSBridgeError.unavailableOnThisMacOS(
                "word markers require macOS 14.0 or newer"
            )
        }
        return AVSpeechSynthesisMarker(
            wordRange: avsNSRange(from: payload.textRange),
            atByteSampleOffset: byteOffset
        )
    case .sentence:
        guard #available(macOS 14.0, *) else {
            throw AVSBridgeError.unavailableOnThisMacOS(
                "sentence markers require macOS 14.0 or newer"
            )
        }
        return AVSpeechSynthesisMarker(
            sentenceRange: avsNSRange(from: payload.textRange),
            atByteSampleOffset: byteOffset
        )
    case .paragraph:
        guard #available(macOS 14.0, *) else {
            throw AVSBridgeError.unavailableOnThisMacOS(
                "paragraph markers require macOS 14.0 or newer"
            )
        }
        return AVSpeechSynthesisMarker(
            paragraphRange: avsNSRange(from: payload.textRange),
            atByteSampleOffset: byteOffset
        )
    case .phoneme:
        guard #available(macOS 14.0, *) else {
            throw AVSBridgeError.unavailableOnThisMacOS(
                "phoneme markers require macOS 14.0 or newer"
            )
        }
        guard let phoneme = payload.phoneme else {
            throw AVSBridgeError.invalidArgument("missing phoneme text")
        }
        return AVSpeechSynthesisMarker(
            phonemeString: phoneme,
            atByteSampleOffset: byteOffset
        )
    case .bookmark:
        guard #available(macOS 14.0, *) else {
            throw AVSBridgeError.unavailableOnThisMacOS(
                "bookmark markers require macOS 14.0 or newer"
            )
        }
        guard let bookmarkName = payload.bookmarkName else {
            throw AVSBridgeError.invalidArgument("missing bookmark name")
        }
        return AVSpeechSynthesisMarker(
            bookmarkName: bookmarkName,
            atByteSampleOffset: byteOffset
        )
    }
}

@_cdecl("avs_marker_make_json")
public func avs_marker_make_json(
    _ markerJson: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let payload = try avsDecodeJSON(markerJson, as: AVSMarkerConstructorPayload.self)
        let marker = try avsMarker(from: payload)
        return avsCString(try avsEncodeJSON(avsMarkerPayload(from: marker)))
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return nil
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return nil
    }
}
