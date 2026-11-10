#!/usr/bin/env bash
# Builds target/android/strana.apk: libstrana.so for each ABI plus the manifest, signed with
# the debug key. Needs the Android NDK, build-tools and a platform (android.jar) in
# $ANDROID_HOME (default ~/Android/Sdk), rustup with the Android targets, and assets/
# extracted from the original (they are built into the library).
#
#   ABIS="arm64-v8a" android/build.sh        # phones only; the default also adds x86_64
set -euo pipefail
cd "$(dirname "$0")/.."

SDK=${ANDROID_HOME:-$HOME/Android/Sdk}
NDK=${ANDROID_NDK_ROOT:-$(ls -d "$SDK"/ndk/* | sort -V | tail -1)}
BT=$(ls -d "$SDK"/build-tools/* | sort -V | tail -1)
JAR=$(ls -d "$SDK"/platforms/android-* | sort -V | tail -1)/android.jar
LLVM=$NDK/toolchains/llvm/prebuilt/linux-x86_64/bin
API=26
ABIS=${ABIS:-"arm64-v8a x86_64"}
OUT=target/android
# The system cargo may lack the Android standard library; rustup's has it.
[ -x "$HOME/.cargo/bin/cargo" ] && export PATH="$HOME/.cargo/bin:$PATH"

rm -rf "$OUT"
mkdir -p "$OUT/apk"
for abi in $ABIS; do
    case $abi in
        arm64-v8a) triple=aarch64-linux-android clang=aarch64-linux-android ;;
        x86_64) triple=x86_64-linux-android clang=x86_64-linux-android ;;
        armeabi-v7a) triple=armv7-linux-androideabi clang=armv7a-linux-androideabi ;;
        *) echo "unknown ABI $abi" >&2; exit 1 ;;
    esac
    var=$(echo "$triple" | tr 'a-z-' 'A-Z_')
    export "CARGO_TARGET_${var}_LINKER=$LLVM/$clang$API-clang"
    export "CC_${triple//-/_}=$LLVM/$clang$API-clang" "AR_${triple//-/_}=$LLVM/llvm-ar"
    cargo rustc --release --lib --target "$triple" --crate-type cdylib
    mkdir -p "$OUT/apk/lib/$abi"
    cp "target/$triple/release/libstrana.so" "$OUT/apk/lib/$abi/"
done

"$BT/aapt2" link -o "$OUT/unsigned.apk" --manifest android/AndroidManifest.xml -I "$JAR"
(cd "$OUT/apk" && "$BT/aapt" add -v ../unsigned.apk lib/*/libstrana.so >/dev/null)
"$BT/zipalign" -f -P 16 4 "$OUT/unsigned.apk" "$OUT/aligned.apk"

KS=$HOME/.android/debug.keystore
if [ ! -f "$KS" ]; then
    mkdir -p "$(dirname "$KS")"
    keytool -genkeypair -keystore "$KS" -storepass android -alias androiddebugkey -keypass android \
        -keyalg RSA -keysize 2048 -validity 10000 -dname "CN=Android Debug,O=Android,C=US"
fi
"$BT/apksigner" sign --ks "$KS" --ks-pass pass:android --out "$OUT/strana.apk" "$OUT/aligned.apk"
rm "$OUT/unsigned.apk" "$OUT/aligned.apk"
echo "$OUT/strana.apk"
