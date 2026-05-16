import AVFAudio
import Foundation

@_cdecl("avs_current_language_code")
public func avs_current_language_code() -> UnsafeMutablePointer<CChar>? {
    avsCString(AVSpeechSynthesisVoice.currentLanguageCode())
}

@_cdecl("avs_speech_voices_json")
public func avs_speech_voices_json() -> UnsafeMutablePointer<CChar>? {
    let voices = AVSpeechSynthesisVoice.speechVoices().sorted {
        ($0.language, $0.name, $0.identifier) < ($1.language, $1.name, $1.identifier)
    }
    do {
        return avsCString(try avsEncodeJSON(voices.map(avsVoicePayload)))
    } catch {
        return nil
    }
}

@_cdecl("avs_voices_with_language_json")
public func avs_voices_with_language_json(
    _ language: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let requestedLanguage = try avsRequireString(language, field: "language")
        let normalized = avsNormalizedLanguage(requestedLanguage)
        let voices = AVSpeechSynthesisVoice.speechVoices()
            .filter { avsNormalizedLanguage($0.language) == normalized }
            .sorted { ($0.name, $0.identifier) < ($1.name, $1.identifier) }
        return avsCString(try avsEncodeJSON(voices.map(avsVoicePayload)))
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return nil
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return nil
    }
}

@_cdecl("avs_voice_with_language_json")
public func avs_voice_with_language_json(
    _ language: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let requestedLanguage = language.map { String(cString: $0) }
        guard let voice = AVSpeechSynthesisVoice(language: requestedLanguage) else {
            return nil
        }
        return avsCString(try avsEncodeJSON(avsVoicePayload(from: voice)))
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return nil
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return nil
    }
}

@_cdecl("avs_voice_with_identifier_json")
public func avs_voice_with_identifier_json(
    _ identifier: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let identifier = try avsRequireString(identifier, field: "identifier")
        guard let voice = AVSpeechSynthesisVoice(identifier: identifier) else {
            return nil
        }
        return avsCString(try avsEncodeJSON(avsVoicePayload(from: voice)))
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return nil
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return nil
    }
}

@_cdecl("avs_alex_voice_identifier")
public func avs_alex_voice_identifier() -> UnsafeMutablePointer<CChar>? {
    avsCString(AVSpeechSynthesisVoiceIdentifierAlex)
}
