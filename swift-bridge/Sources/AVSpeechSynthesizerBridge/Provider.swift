import AVFAudio
import Foundation

struct AVSProviderVoiceConfigPayload: Codable {
    var name: String
    var identifier: String
    var primaryLanguages: [String]
    var supportedLanguages: [String]
}

final class AVSProviderVoiceBox: NSObject {
    let voice: AVSpeechSynthesisProviderVoice

    init(voice: AVSpeechSynthesisProviderVoice) {
        self.voice = voice
        super.init()
    }
}

final class AVSProviderRequestBox: NSObject {
    let request: AVSpeechSynthesisProviderRequest

    init(request: AVSpeechSynthesisProviderRequest) {
        self.request = request
        super.init()
    }
}

private func avsProviderVoiceBox(_ token: UnsafeMutableRawPointer?) throws -> AVSProviderVoiceBox {
    guard let token else {
        throw AVSBridgeError.invalidArgument("missing provider voice token")
    }
    return avsBorrow(token)
}

private func avsProviderRequestBox(_ token: UnsafeMutableRawPointer?) throws -> AVSProviderRequestBox {
    guard let token else {
        throw AVSBridgeError.invalidArgument("missing provider request token")
    }
    return avsBorrow(token)
}

private func avsProviderVoicePayload(from voice: AVSpeechSynthesisProviderVoice) -> AVSProviderVoicePayload {
    AVSProviderVoicePayload(
        name: voice.name,
        identifier: voice.identifier,
        primaryLanguages: voice.primaryLanguages,
        supportedLanguages: voice.supportedLanguages,
        voiceSize: voice.voiceSize,
        version: voice.version,
        gender: Int(voice.gender.rawValue),
        age: Int(voice.age)
    )
}

@_cdecl("avs_provider_voice_new_json")
public func avs_provider_voice_new_json(
    _ configJson: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let config = try avsDecodeJSON(configJson, as: AVSProviderVoiceConfigPayload.self)
        let voice = AVSpeechSynthesisProviderVoice(
            name: config.name,
            identifier: config.identifier,
            primaryLanguages: config.primaryLanguages,
            supportedLanguages: config.supportedLanguages
        )
        return avsRetain(AVSProviderVoiceBox(voice: voice))
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return nil
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return nil
    }
}

@_cdecl("avs_provider_voice_release")
public func avs_provider_voice_release(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    avsRelease(token)
}

@_cdecl("avs_provider_voice_snapshot_json")
public func avs_provider_voice_snapshot_json(
    _ token: UnsafeMutableRawPointer?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let voice = try avsProviderVoiceBox(token).voice
        return avsCString(try avsEncodeJSON(avsProviderVoicePayload(from: voice)))
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return nil
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return nil
    }
}

@_cdecl("avs_provider_voice_set_voice_size")
public func avs_provider_voice_set_voice_size(
    _ token: UnsafeMutableRawPointer?,
    _ voiceSize: Int64,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let voice = try avsProviderVoiceBox(token).voice
        voice.voiceSize = voiceSize
        return AVS_OK
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return AVS_UNKNOWN
    }
}

@_cdecl("avs_provider_voice_set_version")
public func avs_provider_voice_set_version(
    _ token: UnsafeMutableRawPointer?,
    _ version: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let voice = try avsProviderVoiceBox(token).voice
        voice.version = try avsRequireString(version, field: "version")
        return AVS_OK
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return AVS_UNKNOWN
    }
}

@_cdecl("avs_provider_voice_set_gender")
public func avs_provider_voice_set_gender(
    _ token: UnsafeMutableRawPointer?,
    _ gender: Int64,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let rawGender = AVSpeechSynthesisVoiceGender(rawValue: Int(gender)) else {
            throw AVSBridgeError.invalidArgument("unknown provider voice gender: \(gender)")
        }
        let voice = try avsProviderVoiceBox(token).voice
        voice.gender = rawGender
        return AVS_OK
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return AVS_UNKNOWN
    }
}

@_cdecl("avs_provider_voice_set_age")
public func avs_provider_voice_set_age(
    _ token: UnsafeMutableRawPointer?,
    _ age: Int64,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let voice = try avsProviderVoiceBox(token).voice
        voice.age = Int(age)
        return AVS_OK
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return AVS_UNKNOWN
    }
}

@_cdecl("avs_provider_voice_update_speech_voices")
public func avs_provider_voice_update_speech_voices(
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    _ = outErrorMessage
    AVSpeechSynthesisProviderVoice.updateSpeechVoices()
    return AVS_OK
}

@_cdecl("avs_provider_request_new")
public func avs_provider_request_new(
    _ voiceToken: UnsafeMutableRawPointer?,
    _ ssmlRepresentation: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let voice = try avsProviderVoiceBox(voiceToken).voice
        let ssmlRepresentation = try avsRequireString(
            ssmlRepresentation,
            field: "provider request SSML representation"
        )
        let request = AVSpeechSynthesisProviderRequest(
            ssmlRepresentation: ssmlRepresentation,
            voice: voice
        )
        return avsRetain(AVSProviderRequestBox(request: request))
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return nil
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return nil
    }
}

@_cdecl("avs_provider_request_release")
public func avs_provider_request_release(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    avsRelease(token)
}

@_cdecl("avs_provider_request_ssml_representation")
public func avs_provider_request_ssml_representation(
    _ token: UnsafeMutableRawPointer?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let request = try avsProviderRequestBox(token).request
        return avsCString(request.ssmlRepresentation)
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return nil
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return nil
    }
}

@_cdecl("avs_provider_request_copy_voice")
public func avs_provider_request_copy_voice(
    _ token: UnsafeMutableRawPointer?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let request = try avsProviderRequestBox(token).request
        return avsRetain(AVSProviderVoiceBox(voice: request.voice))
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return nil
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return nil
    }
}
