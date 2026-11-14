# Builds everything the Go, Python and Java bindings sync from this crate:
#
#   dist/xhandler.wasm          the WebAssembly module
#   dist/VERSION                version reported by the `version` op
#   dist/xhandler.schema.json   JSON Schema of the models
#   dist/models/                model types generated for each language
#   dist/conformance/           cases every binding must pass
#
# Run from the workspace root: make -C xhandler-wasm dist

ROOT := $(abspath ..)
DIST := $(ROOT)/dist
DUMP := $(ROOT)/target/xbuilder-input-dump
JAVA_MODELS_PACKAGE := io.github.project.openubl.xbuilder.models

.PHONY: dist wasm schema models conformance clean

dist: wasm schema models conformance

wasm:
	cd $(ROOT) && cargo build -p xhandler-wasm --profile wasm --target wasm32-wasip1
	mkdir -p $(DIST)
	cp $(ROOT)/target/wasm32-wasip1/wasm/xhandler_wasm.wasm $(DIST)/xhandler.wasm
	cd $(ROOT) && cargo pkgid -p xhandler-wasm | sed 's/.*[#@]//' > $(DIST)/VERSION

schema:
	mkdir -p $(DIST)
	cd $(ROOT) && cargo run -q -p xhandler-wasm --features schema --example schema > $(DIST)/xhandler.schema.json

models: schema
	rm -rf $(DIST)/models && mkdir -p $(DIST)/models/go $(DIST)/models/python
	python3 codegen.py $(DIST)/xhandler.schema.json go $(DIST)/models/go/models_gen.go
	# gofmt when Go is installed; the Go repository formats it on sync anyway
	command -v gofmt >/dev/null && gofmt -w $(DIST)/models/go/models_gen.go || true
	python3 codegen.py $(DIST)/xhandler.schema.json python $(DIST)/models/python/models.py
	python3 codegen.py $(DIST)/xhandler.schema.json java $(DIST)/models/java $(JAVA_MODELS_PACKAGE)

conformance:
	rm -rf $(DUMP) $(DIST)/conformance
	cd $(ROOT) && XBUILDER_SKIP_SUNAT=true XBUILDER_DUMP_INPUT=$(DUMP) cargo test -q -p xhandler --test xbuilder
	cd $(ROOT) && cargo run -q -p xhandler-wasm --example corpus -- \
		$(DUMP) $(ROOT) $(DIST)/conformance

clean:
	rm -rf $(DIST) $(DUMP)
