build: ../../build/lambda-canvas
.PHONY: build

test: build
	../../build/lambda-canvas ../../data/part-1-examples/example-01.desc
.PHONY: test

clean:
	rm ../../build/lambda-canvas
.PHONY: clean

../../build/lambda-canvas: main.js lambda.js
	npm install
	xshar --output=../../build/lambda-canvas \
		--command="nodejs $${XSHAR_ROOT}$$(pwd)/main.js" \
		main.js lambda.js node_modules

lambda.js: ../lambda.js
	cp ../lambda.js lambda.js
