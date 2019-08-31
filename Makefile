doc:
	cargo doc --no-deps --release
	rm -rf docs
	mv target/doc docs
