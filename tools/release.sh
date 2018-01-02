#!/bin/sh

set -e

CRATE_NAME="sesstype-cli"
CRATE_ROOT=$(dirname $(dirname $(readlink -f "$0"))..) # tools/..
TARGET=$(rustup show | grep "Default host" | cut -d' ' -f3) # Target triple
VERSION=$(git describe --tags)
PKG_DIRNAME="${CRATE_NAME}_${VERSION}"
PKG_PATH="${CRATE_ROOT}/${PKG_DIRNAME}"
ARCHIVE_NAME="${CRATE_NAME}_${VERSION}_${TARGET}"

BINARIES="sesstype-cli" # space separated list of binaries to include

echo -n "Create release ${PKG_DIRNAME} for ${TARGET}? [Y/n] "
read CREATE
if echo "$CREATE" | grep -iq "^n"; then
    exit 0
fi

cargo build --release
mkdir -p ${PKG_PATH}/bin
for BINARY in $BINARIES; do
    cp -v "target/release/${BINARY}" "${PKG_PATH}/bin"
done

cp -r -v "README.md" "${PKG_PATH}"
cp -r -v "LICENSE"   "${PKG_PATH}"
cp -r -v "examples"  "${PKG_PATH}"

tar -C "${CRATE_ROOT}" -czvf "${ARCHIVE_NAME}.tgz" "${PKG_DIRNAME}"
