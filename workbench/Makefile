HAMMER_VERSION ?= v0.1.0
TARGET_ARCH ?= x86_64-unknown-linux-musl_workbench

production:
	make fetch-hammer
	make build-hammer
	zola build
.PHONY: production

preview:
	make fetch-hammer
	make build-hammer
	zola build --base-url ${BASE_URL}
.PHONY: preview

fetch-hammer:
	curl -L -o hammer https://github.com/arnau/data-standards-authority/releases/download/${HAMMER_VERSION}/${ARTIFACT_NAME}
	chmod +x hammer
.PHONY: fetch-hammer

build-hammer:
	RUST_LOG=info ./hammer build -i ../corpus -o ./content
.PHONY: build-hammer
