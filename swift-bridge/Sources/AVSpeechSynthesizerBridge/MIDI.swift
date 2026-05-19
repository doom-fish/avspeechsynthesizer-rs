import AVFAudio
import Foundation

private func avsMIDIFileURL(
    _ pathPointer: UnsafePointer<CChar>?,
    field: String,
    requireExists: Bool
) throws -> URL {
    let path = try avsRequireString(pathPointer, field: field)
    if requireExists && !FileManager.default.fileExists(atPath: path) {
        throw AVSBridgeError.io("\(field) does not exist: \(path)")
    }
    return URL(fileURLWithPath: path)
}

private func avsMIDISoundBankURL(_ pathPointer: UnsafePointer<CChar>?) throws -> URL? {
    guard let pathPointer else {
        return nil
    }
    return try avsMIDIFileURL(pathPointer, field: "sound bank path", requireExists: true)
}

@_cdecl("avs_midi_player_new_with_contents_of_path")
public func avs_midi_player_new_with_contents_of_path(
    _ midiPath: UnsafePointer<CChar>?,
    _ soundBankPath: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let midiURL = try avsMIDIFileURL(midiPath, field: "MIDI file path", requireExists: true)
        let soundBankURL = try avsMIDISoundBankURL(soundBankPath)
        let player = try AVMIDIPlayer(contentsOf: midiURL, soundBankURL: soundBankURL)
        return avsRetain(player)
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return nil
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return nil
    }
}

@_cdecl("avs_midi_player_new_with_data")
public func avs_midi_player_new_with_data(
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ soundBankPath: UnsafePointer<CChar>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        guard len >= 0 else {
            throw AVSBridgeError.invalidArgument("MIDI byte length must be non-negative")
        }
        let soundBankURL = try avsMIDISoundBankURL(soundBankPath)
        let data = bytes.map { Data(bytes: $0, count: len) } ?? Data()
        let player = try AVMIDIPlayer(data: data, soundBankURL: soundBankURL)
        return avsRetain(player)
    } catch let error as AVSBridgeError {
        outErrorMessage?.pointee = avsCString(error.description)
        return nil
    } catch {
        outErrorMessage?.pointee = avsCString(error.localizedDescription)
        return nil
    }
}

@_cdecl("avs_midi_player_release")
public func avs_midi_player_release(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    avsRelease(token)
}

@_cdecl("avs_midi_player_prepare_to_play")
public func avs_midi_player_prepare_to_play(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    let player: AVMIDIPlayer = avsBorrow(token)
    player.prepareToPlay()
}

@_cdecl("avs_midi_player_play")
public func avs_midi_player_play(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    let player: AVMIDIPlayer = avsBorrow(token)
    player.play(nil)
}

@_cdecl("avs_midi_player_stop")
public func avs_midi_player_stop(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    let player: AVMIDIPlayer = avsBorrow(token)
    player.stop()
}

@_cdecl("avs_midi_player_duration")
public func avs_midi_player_duration(_ token: UnsafeMutableRawPointer?) -> Double {
    guard let token else { return 0 }
    let player: AVMIDIPlayer = avsBorrow(token)
    return player.duration
}

@_cdecl("avs_midi_player_is_playing")
public func avs_midi_player_is_playing(_ token: UnsafeMutableRawPointer?) -> Bool {
    guard let token else { return false }
    let player: AVMIDIPlayer = avsBorrow(token)
    return player.isPlaying
}

@_cdecl("avs_midi_player_rate")
public func avs_midi_player_rate(_ token: UnsafeMutableRawPointer?) -> Float {
    guard let token else { return 0 }
    let player: AVMIDIPlayer = avsBorrow(token)
    return player.rate
}

@_cdecl("avs_midi_player_set_rate")
public func avs_midi_player_set_rate(_ token: UnsafeMutableRawPointer?, _ rate: Float) {
    guard let token else { return }
    let player: AVMIDIPlayer = avsBorrow(token)
    player.rate = rate
}

@_cdecl("avs_midi_player_current_position")
public func avs_midi_player_current_position(_ token: UnsafeMutableRawPointer?) -> Double {
    guard let token else { return 0 }
    let player: AVMIDIPlayer = avsBorrow(token)
    return player.currentPosition
}

@_cdecl("avs_midi_player_set_current_position")
public func avs_midi_player_set_current_position(
    _ token: UnsafeMutableRawPointer?,
    _ position: Double
) {
    guard let token else { return }
    let player: AVMIDIPlayer = avsBorrow(token)
    player.currentPosition = position
}

@_cdecl("avs_midi_channel_event_new")
public func avs_midi_channel_event_new(
    _ channel: UInt32,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 13.0, *) else {
        outErrorMessage?.pointee = avsCString("AVMIDIChannelEvent requires macOS 13+")
        return nil
    }
    let event = AVMIDIChannelEvent()
    event.channel = channel
    return avsRetain(event)
}

@_cdecl("avs_midi_channel_event_release")
public func avs_midi_channel_event_release(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    avsRelease(token)
}

@_cdecl("avs_midi_channel_event_channel")
public func avs_midi_channel_event_channel(_ token: UnsafeMutableRawPointer?) -> UInt32 {
    guard #available(macOS 13.0, *), let token else { return 0 }
    let event: AVMIDIChannelEvent = avsBorrow(token)
    return event.channel
}

@_cdecl("avs_midi_channel_event_set_channel")
public func avs_midi_channel_event_set_channel(
    _ token: UnsafeMutableRawPointer?,
    _ channel: UInt32
) {
    guard #available(macOS 13.0, *), let token else { return }
    let event: AVMIDIChannelEvent = avsBorrow(token)
    event.channel = channel
}
