# Chromium Profile Picker

A cross-platform good-looking profile picker for Chromium-based browsers, written in Rust.

<img width="600" alt="image" src="https://github.com/user-attachments/assets/c3eef927-7363-484c-948d-f8cf447b1633" />

## Supported browsers

| Feature flag | Browser |
|---|---|
| `chrome` *(default)* | Google Chrome |
| `chromium` | Chromium |
| `brave` | Brave Browser |
| `edge` | Microsoft Edge |
| `vivaldi` | Vivaldi |
| `opera` | Opera |
| `arc` | Arc *(macOS / Windows only)* |
| `helium` | Helium |

## Installation

On Linux and macOS, run:

```sh
curl -fsSL https://raw.githubusercontent.com/donnybeelo/chromium-profile-picker/main/install.sh | sh
```

On Windows, run this in PowerShell:

```powershell
irm https://raw.githubusercontent.com/donnybeelo/chromium-profile-picker/main/install.ps1 | iex
```

Both ask which browser you use, install `chromium-profile-picker` for your user, and update it when ran again.

To skip the question, pass the browser from the table above:

```sh
curl -fsSL https://raw.githubusercontent.com/donnybeelo/chromium-profile-picker/main/install.sh | sh -s -- helium
```

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/donnybeelo/chromium-profile-picker/main/install.ps1))) helium
```

To build from source instead:

```bash
cargo install --git https://github.com/donnybeelo/chromium-profile-picker --no-default-features --features helium
```

Only one feature should be active at a time.

### Linux

Locate your `.desktop` file for your browser and set `chromium-profile-picker` as the `Exec` command. Do the same for the "New Window" desktop action if it exists.

### Windows

The install script does this for you. If you built from source, run this once after installing to register the app and open the Default apps settings page:

```powershell
chromium-profile-picker --set-default
```

Find the app in the list (e.g. "Microsoft Edge Profile Picker") and set it as the default browser. The app registers itself automatically on every launch, so re-running `--set-default` after an update is enough to keep the registration current.

## Configuration

On first run the picker auto-detects the browser binary and profile directory and writes a config file to:

- **Linux:** `~/.config/net.donnybeelo.chromium-profile-picker/browser.json`
- **macOS:** `~/Library/Application Support/net.donnybeelo.chromium-profile-picker/browser.json`
- **Windows:** `%APPDATA%\net.donnybeelo.chromium-profile-picker\browser.json`

Edit this file to override any auto-detected paths. The config is regenerated automatically when you switch browser features.

## Build from source

```bash
cargo build --release --no-default-features --features helium
```

The release binary will be in:

```bash
./target/release/chromium-profile-picker
```

### Run

```bash
cargo run --no-default-features --features helium -- "https://example.com"
```

Or without a URL:

```bash
cargo run --no-default-features --features helium
```

## Environment variables

- `BROWSER_BIN` — override the browser executable path
- `BROWSER_CONFIG_DIR` — override the browser profile directory
