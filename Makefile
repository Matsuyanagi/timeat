# cargo run -- --unixtime-str 1706370267 1706370280

.PHONY: run
run:
	cargo run -- 1706370267 1706370280 1706375767

.PHONY: build
build:
	cargo build
	
.PHONY: release
release:
	cargo build --release
	
	
	
