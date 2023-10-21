# amp

Library to programmatically control Sony Headphones (WH-1000XM3, WH-1000XM4, etc.) through Bluetooth Low Energy.

## Status

This project is in early development.

## Library Features

- **Provide API for connecting to Sony Headphones**
  - List nearby devices
  - Connect
  - Perform Handshake
- **Provide list of headphones' capabilities, and ways to control them**
  - Get/Set Ambience Sound Mode (ANC)
  - Set Equalizer
  - Reconfigure Buttons
- **Provide information**
  - Model name/number
  - Battery level
  - Firmware version
- **Platform independency**
  - Language agnostic through C-ABI
  - Mostly platform agnostic through Rust and [`btleplug`](https://github.com/deviceplug/btleplug), which supports Windows, macOS, iOS, Linux, and Android

## Supported Headphones

None yet.

## License

amp is provided to you under the GPL-3.0 license. See "LICENSE" for more information.
