build:
	npm install
	xshar --output=../build/solution-check \
		--command="nodejs $${XSHAR_ROOT}$$(pwd)/main.js" *.js node_modules
.PHONY: build
