#!/bin/bash
#
# Downloads and extracts the X-Plane SDK for plugin development
#

set -e

SDK_VERSION="401"
SDK_URL="https://developer.x-plane.com/wp-content/plugins/code-sample-generation/sdk_zip_files/XPSDK${SDK_VERSION}.zip"
SDK_DIR="$(dirname "$0")/sdk"

echo "=== X-Plane SDK Setup ==="

# Check if already exists
if [ -f "${SDK_DIR}/CHeaders/XPLM/XPLMPlugin.h" ]; then
    echo "SDK already installed at ${SDK_DIR}"
    exit 0
fi

# Create temp directory
TMPDIR=$(mktemp -d)
trap "rm -rf ${TMPDIR}" EXIT

echo "Downloading X-Plane SDK ${SDK_VERSION}..."
curl -L -o "${TMPDIR}/sdk.zip" "${SDK_URL}"

echo "Extracting..."
unzip -q "${TMPDIR}/sdk.zip" -d "${TMPDIR}"

# Move to correct location
mkdir -p "${SDK_DIR}"
# The zip extracts to a folder named "SDK"
mv "${TMPDIR}/SDK/"* "${SDK_DIR}/"

echo "Verifying installation..."
if [ -f "${SDK_DIR}/CHeaders/XPLM/XPLMPlugin.h" ]; then
    echo "✓ SDK installed successfully to ${SDK_DIR}"
    echo ""
    echo "SDK Contents:"
    ls -la "${SDK_DIR}/"
else
    echo "✗ Installation failed - header files not found"
    exit 1
fi
