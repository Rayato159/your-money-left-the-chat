migrate-up:
	diesel migration run

migrate-down:
	diesel migration revert

build-release:
	cargo build --release --example your_money_left_the_chat