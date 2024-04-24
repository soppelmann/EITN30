base: 
	git pull
	cargo build
	./scripts/setcap.sh
	cargo run -- --base

mobile:	
	git pull
	cargo build
	./scripts/setcap.sh
	cargo run -- --mobile
