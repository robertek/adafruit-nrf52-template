# adafruit-nrf52-template
Adafruit nrf52 rust embassy template

## Dependencies
 * cargo-binutils
 * cargo-generate
 * cargo-make
 * adafruit-nrfutil (or plain python3)

## Installation

```
cargo generate --git https://github.com/robertek/adafruit-nrf52-template
```

## Usage

Building:
```
cargo make
```

Uploading to the device when in DFU mode.

```
cargo make upload
```

## Obtaining adafruit-nrfutil

To download adafruit-nrfutil to your workspace directoy:

```
cargo make nrfutil_install
```

And than add it to the path:

```
export PATH=./adafruit-nrfutil/bin:$PATH
```
