check:
	scripts/check.sh

build:
	cargo build

# TODO: Release
run-native:
	cargo run

run-native-release:
	cargo run --release

# TODO: Release
run-web: check
	cd dist
	python3 -m http.server 6969
