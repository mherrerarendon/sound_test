flutter_rust_bridge_codegen --rust-input tuner-rs/src/api.rs --dart-output lib/api.dart --c-output ios/Runner/api.h
if [ $? -ne 0 ]; then
    echo "flutter rust bridge codegen failed"
    exit 1
fi

cd tuner-rs && cargo build && cargo lipo && cd ..
if [ $? -ne 0 ]; then
    echo "build step failed"
    exit 1
fi

cp tuner-rs/target/universal/debug/libtuner_rs.a ios/Runner/libtuner_rs.a
if [ $? -ne 0 ]; then
    echo "copy step failed"
    exit 1
fi
echo "All done!"