# MacroMate

Command-line macro recorder and player for Linux.

## Features

- Record keyboard and mouse input events (keys, buttons, movement, wheel)
- Simple scripting language
- Uses kernel evdev API - works on Wayland and X11

## Requirements

- Linux kernel with evdev support
- Rust 1.85+ (edition 2024)
- Root access or permissions to read `/dev/input/event*` devices

## Installation

```bash
cargo build --release
sudo cp target/release/macromate /usr/local/bin/
```

## Usage

### Record a macro

```bash
macromate record my_macro.txt
```

### Play back a macro

```bash
# macromate play my_macro.txt
```

## File Format

Coming soon!

## Future Enhancements

- [x] Hotkey detection to start/stop recording
- [x] Repeat/loop playback
- [ ] Configurable hotkeys (currently F1 is hardcoded)
- [ ] Better scripting language

## License

AGPL-3.0-or-later
