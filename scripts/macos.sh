cargo +nightly build -Z build-std=std,panic_abort --target aarch64-apple-darwin --release &&\
rm /usr/local/bin/tt && ln target/aarch64-apple-darwin/release/tt /usr/local/bin/tt