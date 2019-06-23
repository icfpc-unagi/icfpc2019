build: ../../build/solution_checker
.PHONY: build

test: build
	cd ../../build && bash $(shell pwd)/test.sh ./solution_checker
.PHONY: test

clean:
	rm ../../build/solution_checker
.PHONY: clean

../../build/solution_checker: main.js lambda.js
	npm install
	xshar --output=../../build/solution_checker \
		--command='nodejs $${XSHAR_ROOT}'"$$(pwd)"'/main.js' \
		main.js lambda.js node_modules

lambda.js: ../lambda.js
	cp ../lambda.js lambda.js
