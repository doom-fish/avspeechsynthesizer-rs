import AVFAudio
import Foundation

struct AVSAttributedSpeechAttributeRunPayload: Codable {
    var range: AVSRangePayload
    var attributesJson: String
}

enum AVSUtteranceKind: String, Codable {
    case plainText
    case attributedText
    case ssml
}

struct AVSAttributedSpeechStringPayload: Codable {
    var text: String
    var runs: [AVSAttributedSpeechAttributeRunPayload]
}

struct AVSUtterancePayload: Codable {
    var kind: AVSUtteranceKind
    var speechString: String
    var attributedSpeechString: AVSAttributedSpeechStringPayload?
    var ssmlRepresentation: String?
    var voice: AVSVoicePayload?
    var rate: Float
    var pitchMultiplier: Float
    var volume: Float
    var prefersAssistiveTechnologySettings: Bool
    var preUtteranceDelay: Double
    var postUtteranceDelay: Double
}

private func avsAttributedSpeechString(
    from payload: AVSAttributedSpeechStringPayload
) throws -> NSAttributedString {
    let attributedString = NSMutableAttributedString(string: payload.text)
    let utf16Length = payload.text.utf16.count

    for run in payload.runs {
        let range = NSRange(location: run.range.location, length: run.range.length)
        guard range.location >= 0, range.length >= 0, NSMaxRange(range) <= utf16Length else {
            throw AVSBridgeError.invalidArgument("speech attribute range is out of bounds")
        }

        let attributes = try avsJSONObjectDictionary(
            from: run.attributesJson,
            field: "speech attribute run"
        ).reduce(into: [NSAttributedString.Key: Any]()) { partialResult, item in
            partialResult[NSAttributedString.Key(item.key)] = item.value
        }
        attributedString.addAttributes(attributes, range: range)
    }

    return attributedString
}

private func avsAttributedSpeechStringPayload(
    from attributedString: NSAttributedString
) -> AVSAttributedSpeechStringPayload {
    var runs: [AVSAttributedSpeechAttributeRunPayload] = []
    let fullRange = NSRange(location: 0, length: attributedString.length)

    attributedString.enumerateAttributes(in: fullRange, options: []) { attributes, range, _ in
        guard !attributes.isEmpty else { return }

        let rawAttributes = attributes.reduce(into: [String: Any]()) { partialResult, item in
            partialResult[item.key.rawValue] = item.value
        }
        guard let attributesJson = try? avsJSONString(fromJSONObject: rawAttributes) else {
            return
        }

        runs.append(
            AVSAttributedSpeechAttributeRunPayload(
                range: avsRangePayload(from: range),
                attributesJson: attributesJson
            )
        )
    }

    return AVSAttributedSpeechStringPayload(text: attributedString.string, runs: runs)
}

func avsUtterance(from payload: AVSUtterancePayload) throws -> AVSpeechUtterance {
    let utterance: AVSpeechUtterance
    switch payload.kind {
    case .plainText:
        utterance = AVSpeechUtterance(string: payload.speechString)
    case .attributedText:
        guard let attributedSpeechString = payload.attributedSpeechString else {
            throw AVSBridgeError.invalidArgument("missing attributed speech string payload")
        }
        utterance = AVSpeechUtterance(attributedString: try avsAttributedSpeechString(from: attributedSpeechString))
    case .ssml:
        guard #available(macOS 13.0, *) else {
            throw AVSBridgeError.unavailableOnThisMacOS(
                "SSML utterances require macOS 13.0 or newer"
            )
        }
        let ssmlRepresentation = payload.ssmlRepresentation ?? payload.speechString
        guard let ssmlUtterance = AVSpeechUtterance(ssmlRepresentation: ssmlRepresentation) else {
            throw AVSBridgeError.invalidArgument("invalid SSML representation")
        }
        utterance = ssmlUtterance
    }

    utterance.voice = avsVoice(from: payload.voice)
    utterance.rate = payload.rate
    utterance.pitchMultiplier = payload.pitchMultiplier
    utterance.volume = payload.volume
    utterance.prefersAssistiveTechnologySettings = payload.prefersAssistiveTechnologySettings
    utterance.preUtteranceDelay = payload.preUtteranceDelay
    utterance.postUtteranceDelay = payload.postUtteranceDelay
    return utterance
}

func avsUtterancePayload(
    from utterance: AVSpeechUtterance,
    originalKind: AVSUtteranceKind? = nil,
    originalSSMLRepresentation: String? = nil
) -> AVSUtterancePayload {
    let attributedSpeechString = avsAttributedSpeechStringPayload(from: utterance.attributedSpeechString)
    let kind: AVSUtteranceKind
    if originalKind == .ssml {
        kind = .ssml
    } else if attributedSpeechString.runs.isEmpty {
        kind = .plainText
    } else {
        kind = .attributedText
    }

    return AVSUtterancePayload(
        kind: kind,
        speechString: utterance.speechString,
        attributedSpeechString: attributedSpeechString.runs.isEmpty ? nil : attributedSpeechString,
        ssmlRepresentation: originalKind == .ssml ? originalSSMLRepresentation : nil,
        voice: utterance.voice.map(avsVoicePayload),
        rate: utterance.rate,
        pitchMultiplier: utterance.pitchMultiplier,
        volume: utterance.volume,
        prefersAssistiveTechnologySettings: utterance.prefersAssistiveTechnologySettings,
        preUtteranceDelay: utterance.preUtteranceDelay,
        postUtteranceDelay: utterance.postUtteranceDelay
    )
}

@_cdecl("avs_utterance_minimum_speech_rate")
public func avs_utterance_minimum_speech_rate() -> Float {
    AVSpeechUtteranceMinimumSpeechRate
}

@_cdecl("avs_utterance_default_speech_rate")
public func avs_utterance_default_speech_rate() -> Float {
    AVSpeechUtteranceDefaultSpeechRate
}

@_cdecl("avs_utterance_maximum_speech_rate")
public func avs_utterance_maximum_speech_rate() -> Float {
    AVSpeechUtteranceMaximumSpeechRate
}

@_cdecl("avs_utterance_ipa_notation_attribute_name")
public func avs_utterance_ipa_notation_attribute_name() -> UnsafeMutablePointer<CChar>? {
    avsCString(AVSpeechSynthesisIPANotationAttribute)
}

@_cdecl("avs_utterance_roundtrip_json")
public func avs_utterance_roundtrip_json(
    _ utteranceJson: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let payload = try avsDecodeJSON(utteranceJson, as: AVSUtterancePayload.self)
        let utterance = try avsUtterance(from: payload)
        let roundtrip = avsUtterancePayload(
            from: utterance,
            originalKind: payload.kind == .ssml ? .ssml : nil,
            originalSSMLRepresentation: payload.ssmlRepresentation ?? payload.speechString
        )
        return avsCString(try avsEncodeJSON(roundtrip))
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return nil
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return nil
    }
}
