all:
	@make build && make run

clean:
	@rm -rf target

build: src/main.rs
	@cargo build

run: src/main.rs
	@cargo run