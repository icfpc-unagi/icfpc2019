build: ../../build/puzzle_checker
.PHONY: build

test: build
	../../build/puzzle_checker ../../data/part-1-examples/example-01.desc
.PHONY: test

clean:
	rm ../../build/puzzle_checker
.PHONY: clean

../../build/puzzle_checker: main.js lambda.js
	npm install
	xshar --output=../../build/puzzle_checker \
		--command="nodejs $${XSHAR_ROOT}$$(pwd)/main.js" \
		main.js lambda.js node_modules

lambda.js: ../lambda.js
	cp ../lambda.js lambda.js
