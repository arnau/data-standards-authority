HAMMER_VERSION ?= v0.1.0
TARGET_ARCH ?= linux-amd64

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
	curl -L -o hammer https://github.com/arnau/data-standards-authority/releases/download/${HAMMER_VERSION}/workbench-${TARGET_ARCH}
	chmod +x hammer
.PHONY: fetch-hammer

build-hammer:
	RUST_LOG=info ./hammer build -i ../corpus -o ./content
.PHONY: build-hammer
