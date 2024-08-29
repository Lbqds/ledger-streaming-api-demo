version := 3.34.0
ledger_app_builder = ghcr.io/ledgerhq/ledger-app-builder/ledger-app-builder:$(version)
ledger_app_dev_tools = ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools:$(version)

release:
	@make _release device=stax
	@make _release device=flex

_release:
	@docker run --rm -v $(shell pwd):/app -v ledger-demo:/opt/.cargo $(ledger_app_builder) \
		bash -c " \
			echo 'Building $(device) app' && \
			RUST_BACKTRACE=1 cargo ledger build $(device) -- -Z unstable-options && \
			cp ./target/$(device)/release/demo.hex ../$(device).hex && \
			mv ./target/$(device)/release/app_$(device).json ../$(device).json \
		"

check:
	@docker run --rm -v $(shell pwd):/app -v ledger-demo:/opt/.cargo $(ledger_app_builder) \
		bash -c " \
			echo 'Cargo fmt' && \
			cargo fmt --all -- --check && \
			echo 'Cargo clippy' && \
			cargo +nightly-2023-11-10 clippy --target=stax && \
			cargo install cargo-audit && cargo audit && \
			cargo install --locked cargo-deny && cargo +nightly-2023-11-10 deny check \
		"

_run-speculos:
	docker run --rm -it -v $(shell pwd):/app --publish 25000:5000 --publish 9999:9999 -e DISPLAY='host.docker.internal:0' \
		-v '/tmp/.X11-unix:/tmp/.X11-unix' --privileged $(ledger_app_dev_tools) \
		speculos -m $(device) /app/target/$(path)/release/demo

run-speculos:
	@make run-speculos-stax

run-speculos-stax:
	@make _run-speculos device=stax path=stax

run-speculos-flex:
	@make _run-speculos device=flex path=flex

clean:
	cargo clean

.PHONY: release clean

