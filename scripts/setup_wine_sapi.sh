#!/bin/bash
#
# SayIntentions Wine Setup Script
# Attempts to install SAPI voice packages for Wine
#

set -e

# Use the default Wine prefix or allow override
WINEPREFIX="${WINEPREFIX:-$HOME/.wine}"
export WINEPREFIX

echo "=========================================="
echo "SayIntentions Wine SAPI Setup"
echo "=========================================="
echo ""
echo "Wine Prefix: $WINEPREFIX"
echo ""

# Check for winetricks
if ! command -v winetricks &> /dev/null; then
    echo "ERROR: winetricks not found"
    echo "Install with: sudo apt install winetricks"
    exit 1
fi

echo "[1/5] Installing .NET Framework 4.8..."
winetricks -q dotnet48 || echo "Warning: dotnet48 install failed, trying 4.0..."
winetricks -q dotnet40 || echo "Warning: dotnet40 also failed"

echo ""
echo "[2/5] Installing Windows components..."
winetricks -q msxml6 || true
winetricks -q corefonts || true

echo ""
echo "[3/5] Installing Speech SDK..."
winetricks -q speechsdk || echo "Warning: speechsdk install failed"

echo ""
echo "[4/5] Setting Windows version to 10..."
winetricks -q win10 || true

echo ""
echo "[5/5] Checking for SAPI DLL..."
SAPI_PATH="$WINEPREFIX/drive_c/windows/system32/Speech"
if [ -d "$SAPI_PATH" ]; then
    echo "SAPI directory exists: $SAPI_PATH"
    ls -la "$SAPI_PATH/" 2>/dev/null || true
else
    echo "SAPI directory not found"
    echo "Creating Speech directories..."
    mkdir -p "$WINEPREFIX/drive_c/Program Files/Common Files/Microsoft Shared/Speech"
fi

echo ""
echo "=========================================="
echo "Setup complete!"
echo "=========================================="
echo ""
echo "Next steps:"
echo "1. Download Microsoft Speech Platform voices from:"
echo "   https://www.microsoft.com/en-us/download/details.aspx?id=27224"
echo ""
echo "2. Install the voice pack:"
echo "   wine msiexec /i MSSpeech_TTS_en-US_ZiraPro.msi"
echo ""
echo "3. Try running SayIntentions again"
echo ""
echo "If still failing, you may need to manually download the Speech Platform Runtime:"
echo "   https://www.microsoft.com/en-us/download/details.aspx?id=27225"
