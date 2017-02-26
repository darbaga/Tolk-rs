# Tolk-rs
Rust bindings to [Tolk](https://github.com/dkager/tolk), a screen reader abstraction library

## Note
This library is meant to be used in the context of providing a screen reader some text to speak. If you're a sighted developer and want general UI accessibility, this library isn't for you. Please use the more powerful native accessibility APIs provided by the OS. Using this library also means that tools for other disabilities (such as magnifiers, voice dictation, etc.) will not be able to understand your UI.

## Installation
Download tolk.zip from [here](https://ci.appveyor.com/api/projects/dkager/tolk/artifacts/tolk.zip?branch=master) and:
* Unzip it to wherever you want
* Copy bin\\{x86, x64}\tolk.lib to:
  * C:\Users\\{Your Username}\\.multirust\toolchains\\{current toolchain}\lib\rustlib\\{current toolchain}\lib if you're using rustup
  * C:\Program Files\Rust\lib\rustlib\x86_64-pc-windows-msvc\lib if you're using a default rust installation
  * LIB if you have the LIB environment variable defined
* Copy bin\\{x86, x64}\tolk.dll to the directory with your project's Cargo.toml
* Copy everything from the lib directory to the directory with your project's Cargo.toml

(x86 and x64 is the kind of rust binaries you have)
And add this to your Cargo.toml (or use cargo add if you have that):
> tolk = "0.1"

## Documentation
Documentation is auto-generated [here](https://docs.rs/tolk/*/x86_64-pc-windows-msvc/tolk/). You don't really need anything else.

## Todo
* Error handling
* Make speak, output, and braille only work if DetectScreenReader returns a string