install: build
	sudo cp ./target/debug/fdupes-xenon /usr/local/bin/ # debug ?

uninstall:
	sudo rm -rf /usr/local/bin/fdupes-xenon

run:
	cargo run "./for tests/" -r
	# or:
	#cargo build
	#./target/debug/fdupes-xenon <PATH> -r

test:
	cargo test

build:
	cargo build

clean:
	rm -rf ./target

