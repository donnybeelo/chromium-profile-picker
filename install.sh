#!/bin/sh
set -eu

browser="${1:-}"

case "$(uname -s)-$(uname -m)" in
    Linux-x86_64)
        target=x86_64-unknown-linux-gnu
        browsers="chrome chromium brave edge vivaldi opera helium"
        ;;
    Darwin-arm64)
        target=aarch64-apple-darwin
        browsers="chrome chromium brave edge vivaldi opera arc helium"
        ;;
    *)
        echo "No prebuilt chromium-profile-picker for $(uname -s) $(uname -m)." >&2
        echo "Build it from source: cargo install --git https://github.com/donnybeelo/chromium-profile-picker --no-default-features --features <browser>" >&2
        exit 1
        ;;
esac

if [ -z "$browser" ]; then
    i=1
    for b in $browsers; do
        echo "  $i) $b"
        i=$((i + 1))
    done
    printf "Which browser? [1-%s]: " "$((i - 1))"
    read -r choice < /dev/tty
    i=1
    for b in $browsers; do
        [ "$i" = "$choice" ] && browser="$b"
        i=$((i + 1))
    done
fi

case " $browsers " in
    *" $browser "*) ;;
    *)
        echo "Not available here: '$browser'. Choose one of: $browsers" >&2
        exit 1
        ;;
esac

dir="$HOME/.local/bin"
url="https://github.com/donnybeelo/chromium-profile-picker/releases/latest/download/chromium-profile-picker-$browser-$target"
mkdir -p "$dir"
tmp="$dir/.chromium-profile-picker.download"

echo "Downloading chromium-profile-picker for $browser..."
if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$url" -o "$tmp"
else
    wget -qO "$tmp" "$url"
fi
chmod +x "$tmp"
mv "$tmp" "$dir/chromium-profile-picker"

case ":$PATH:" in
    *":$dir:"*) echo "Installed to $dir/chromium-profile-picker" ;;
    *)
        echo "Installed to $dir/chromium-profile-picker, but that folder isn't on your PATH."
        echo "Add this line to your shell's startup file (~/.bashrc, ~/.zshrc or ~/.profile):"
        echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
        ;;
esac
