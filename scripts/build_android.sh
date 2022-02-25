#!/bin/sh
JNI_LIBS=android/app/src/main/jniLibs

echo "Remember to set `crate-type = [\"cdylib\"]` in Cargo.toml"

echo "Cleaning previous build..."
rm -rf $JNI_LIBS
mkdir $JNI_LIBS
mkdir $JNI_LIBS/arm64-v8a
mkdir $JNI_LIBS/armeabi-v7a
mkdir $JNI_LIBS/x86

echo "Running flutter rust bridge codegen..."
flutter_rust_bridge_codegen --rust-input tuner-rs/src/api.rs --dart-output lib/api.dart

echo "Building for Android..."
cd tuner-rs
cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
cd ..


echo "Copying artifacts to $JNI_LIBS..."
cp tuner-rs/target/aarch64-linux-android/release/libtuner_rs.so $JNI_LIBS/arm64-v8a/libtuner_rs.so
cp tuner-rs/target/armv7-linux-androideabi/release/libtuner_rs.so $JNI_LIBS/armeabi-v7a/libtuner_rs.so
cp tuner-rs/target/i686-linux-android/release/libtuner_rs.so $JNI_LIBS/x86/libtuner_rs.so

echo "All done!"