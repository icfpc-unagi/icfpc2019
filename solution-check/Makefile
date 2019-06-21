build: ../build/solution-check
.PHONY: build

../build/solution-check: main.js lambda.js
	npm install
	xshar --output=../build/solution-check \
		--command="nodejs $${XSHAR_ROOT}$$(pwd)/main.js" *.js node_modules
