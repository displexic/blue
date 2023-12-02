# Installation

::: warning Important
This project is still in development, don't expect any of the features to work. I (Slekup) also don't have a macOS device to test on, so macOS support is not guaranteed. If you want to help with macOS support, that would be greatly appreciated.
:::

## On Windows

```powershell
iwr https://blue.slekup.com/install.ps1 -useb | iex
```

## On Linux and macOS

```bash
curl -fsSL https://blue.slekup.com/install.sh | sh -
```

or with `wget`

```bash
wget -qO- https://blue.slekup.com/install.sh | sh -
```

## Crates.io

If you would like to build the source code yourself, installing Blue from [crates.io](https://crates.io/crates/blue-cli) will not include the pre-built binaries.

```bash
cargo install blue-cli
```
