default:
  just --list

build-windows:
	cross build --release --target x86_64-pc-windows-msvc

build-linux:
	cross build --release --target x86_64-unknown-linux-gnu

build: build-linux build-windows
