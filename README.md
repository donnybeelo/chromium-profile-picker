# Helium Profile Picker

A small, beautiful, cross-platform profile picker for Helium, written in Rust.

<img width="600" alt="image" src="https://github.com/user-attachments/assets/c3eef927-7363-484c-948d-f8cf447b1633" />


## Build

```bash
cargo build --release
```

The release binary will be in:

```bash
./target/release/helium-profile-picker
```

## Run

```bash
cargo run -- "https://example.com"
```

Or without a URL:

```bash
cargo run
```

## Environment variables

- `HELIUM_CONFIG_DIR` — override the Helium profile directory
- `HELIUM_BIN` — override the Helium executable path

## Notes

The app reads Helium's `Local State` file to discover profiles and preserve their display order.
If the built-in avatar images can be found from Chrome/Chromium/Edge resource packs, those are extracted and cached locally.
