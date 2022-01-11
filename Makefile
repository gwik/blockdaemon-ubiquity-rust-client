.PHONY: all generate clean

all: clean generate build
generate:
	@echo "Generating code..."
	docker run --rm -v "$$(pwd):/local" \
		--user $(shell id -u):$(shell id -g) \
		openapitools/openapi-generator-cli:v5.2.0 generate -v \
		-i /local/spec/openapi.yaml \
		-g rust \
		-o /local/src/generated \
		-c /local/open-api-conf.yaml \
	  -o /local/generated --additional-properties=generateAliasAsModel=true
	/bin/cp -r generated/docs . # use /bin/cp to prevent aliasing from cp to cp -i

build:
	cargo build

clean_generated:
	@echo "Cleaning up 'generated' folder..."
	rm -rf generated

clean_library:
	@echo "Cleaning up generated library at 'ubiquity/ubiquity_openapi_client'..."
	rm -rf ubiquity/ubiquity_openapi_client

clean: clean_generated clean_library

.PHONY: test
test: clean_generated
	cargo test

.PHONY: examples
examples:
	cd examples/get-block/; pwd; \
      echo "Building get-block example"; \
	  cargo build
	#cd ../../
	cd examples/pagination/; pwd; \
      echo "Building pagination example"; \
	  cargo build;
