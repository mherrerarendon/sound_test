cd tuner-rs && cargo build && cargo lipo && cd ..
cp tuner-rs/target/universal/debug/libtuner_rs.a ios/Runner/libtuner_rs.a
flutter_rust_bridge_codegen --rust-input tuner-rs/src/api.rs --dart-output lib/api.dart --c-output ios/Runner/api.h