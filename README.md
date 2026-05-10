# Helium Profile Picker

A cross-platform good-looking profile picker for the [Helium browser](https://github.com/imputnet/helium), written in Rust.

<img width="600" alt="image" src="https://github.com/user-attachments/assets/c3eef927-7363-484c-948d-f8cf447b1633" />

(Works with any Chromium browser)

## Installation

To install using cargo, run the following:

```bash
cargo install --git https://github.com/donnybeelo/helium-profile-picker
```

Locate your .desktop file for helium, and make it launch `helium-profile-picker` in the `Exec` field. Do the same for the "New Window" desktop action, if it exists.

## Build from source

After cloning the repository, build using cargo:

```bash
cargo build --release
```

The release binary will be in:

```bash
./target/release/helium-profile-picker
```

### Run

```bash
cargo run -- "https://example.com"
```

Or without a URL:

```bash
cargo run
```

## Environment variables

- `HELIUM_CONFIG_DIR` — override the Helium profile directory, e.g. `"~/.config/google-chrome"`
- `HELIUM_BIN` — override the Helium executable path, e.g. `"google-chrome"`
