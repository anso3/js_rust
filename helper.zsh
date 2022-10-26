build() {
	 cross build --target x86_64-unknown-linux-gnu --release
}
zipRustLambda() {
	zip -r9 -j bootstrap.zip ./target/x86_64-unknown-linux-gnu/release/bootstrap
}