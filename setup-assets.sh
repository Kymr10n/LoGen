#!/bin/bash
# Setup script for downloading required font assets

set -e

FONT_DIR="assets/fonts"
FONT_FILE="$FONT_DIR/LiberationSans-Bold.ttf"
FONT_URL="https://github.com/googlefonts/roboto/releases/download/v2.138/roboto-android.zip"

echo "🔧 LoGen Asset Setup"
echo "===================="

# Create font directory if it doesn't exist
mkdir -p "$FONT_DIR"

# Check if font already exists
if [ -f "$FONT_FILE" ]; then
    echo "✅ Font file already exists: $FONT_FILE"
    echo "   File size: $(du -h "$FONT_FILE" | cut -f1)"
    exit 0
fi

echo "⬇️  Downloading Roboto Bold font..."

# Check for required tools
if ! command -v curl &> /dev/null && ! command -v wget &> /dev/null; then
    echo "❌ Error: Neither curl nor wget is installed."
    echo "   Please install one of these tools to download the font."
    exit 1
fi

if ! command -v unzip &> /dev/null; then
    echo "❌ Error: unzip is not installed."
    echo "   Please install unzip to extract the font archive."
    exit 1
fi

# Download and extract font
TMP_ZIP=$(mktemp).zip

if command -v curl &> /dev/null; then
    curl -L -o "$TMP_ZIP" "$FONT_URL" || {
        echo "❌ Failed to download font"
        rm -f "$TMP_ZIP"
        exit 1
    }
else
    wget -O "$TMP_ZIP" "$FONT_URL" || {
        echo "❌ Failed to download font"
        rm -f "$TMP_ZIP"
        exit 1
    }
fi

echo "📦 Extracting font..."
unzip -q -j "$TMP_ZIP" "Roboto-Bold.ttf" -d "$FONT_DIR" || {
    echo "❌ Failed to extract font"
    rm -f "$TMP_ZIP"
    exit 1
}

# Rename to expected name
mv "$FONT_DIR/Roboto-Bold.ttf" "$FONT_FILE"

# Cleanup
rm -f "$TMP_ZIP"

# Verify font file
if [ -f "$FONT_FILE" ]; then
    echo "✅ Font installed successfully: $FONT_FILE"
    echo "   File size: $(du -h "$FONT_FILE" | cut -f1)"
    
    # Verify it's a valid font file
    if command -v file &> /dev/null; then
        file_type=$(file "$FONT_FILE")
        if [[ $file_type == *"TrueType"* ]] || [[ $file_type == *"OpenType"* ]]; then
            echo "   Verified: Valid font file"
        else
            echo "⚠️  Warning: File may not be a valid font"
        fi
    fi
else
    echo "❌ Font installation failed"
    exit 1
fi

echo ""
echo "✨ Setup complete! You can now build the project with 'cargo build'"
