.PHONY: list
list:
	@LC_ALL=C $(MAKE) -pRrq -f $(firstword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/(^|\n)# Files(\n|$$)/,/(^|\n)# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | grep -E -v -e '^[^[:alnum:]]' -e '^$@$$'
# IMPORTANT: The line above must be indented by (at least one) 
#            *actual TAB character* - *spaces* do *not* work.


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

