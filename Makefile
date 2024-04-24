base: 
	git pull
	cargo build
	./scripts/setcap.sh
	./target/aarch64-unknown-linux-gnu/debug/eitn_30 --base

mobile:	
	git pull
	cargo build
	./scripts/setcap.sh
	./target/aarch64-unknown-linux-gnu/debug/eitn_30 --mobile
