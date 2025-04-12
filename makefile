migrate-up:
	diesel migration run

migrate-down:
	diesel migration revert

build-release:
	cargo build --release --example your_money_left_the_chat

let-go:
	chmod +x ./target/release/examples/your_money_left_the_chat
	chmod +x ./src/infrastructure/database/sqlite_data/database.db

tests:
	cargo tarpaulin --out html