base: 
	git pull
	cargo build
	./scripts/setcap.sh
	./target/debug/eitn_30 --base

mobile:	
	git pull
	cargo build
	./scripts/setcap.sh
	./target/debug/eitn_30 --mobile
