import AVFAudio
import Foundation

@_cdecl("avs_personal_voice_authorization_status")
public func avs_personal_voice_authorization_status(
    _ outStatus: UnsafeMutablePointer<Int32>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outStatus else {
        outErrorMessage?.pointee = avsCString("missing output status pointer")
        return AVS_INVALID_ARGUMENT
    }
    guard #available(macOS 14.0, *) else {
        outErrorMessage?.pointee = avsCString(
            "personal voice authorization requires macOS 14.0 or newer"
        )
        return AVS_UNAVAILABLE_ON_THIS_MACOS
    }
    outStatus.pointee = Int32(AVSpeechSynthesizer.personalVoiceAuthorizationStatus.rawValue)
    return AVS_OK
}

@_cdecl("avs_request_personal_voice_authorization")
public func avs_request_personal_voice_authorization(
    _ timeoutSeconds: Int32,
    _ outStatus: UnsafeMutablePointer<Int32>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outStatus else {
        outErrorMessage?.pointee = avsCString("missing output status pointer")
        return AVS_INVALID_ARGUMENT
    }
    guard timeoutSeconds > 0 else {
        outErrorMessage?.pointee = avsCString("timeout must be greater than zero seconds")
        return AVS_INVALID_ARGUMENT
    }
    guard #available(macOS 14.0, *) else {
        outErrorMessage?.pointee = avsCString(
            "personal voice authorization requires macOS 14.0 or newer"
        )
        return AVS_UNAVAILABLE_ON_THIS_MACOS
    }

    let semaphore = DispatchSemaphore(value: 0)
    var resolvedStatus = AVSpeechSynthesizer.personalVoiceAuthorizationStatus

    AVSpeechSynthesizer.requestPersonalVoiceAuthorization { status in
        resolvedStatus = status
        semaphore.signal()
    }

    if semaphore.wait(timeout: .now() + .seconds(Int(timeoutSeconds))) == .timedOut {
        outErrorMessage?.pointee = avsCString(
            "timed out waiting for personal voice authorization"
        )
        return AVS_TIMED_OUT
    }

    outStatus.pointee = Int32(resolvedStatus.rawValue)
    return AVS_OK
}
