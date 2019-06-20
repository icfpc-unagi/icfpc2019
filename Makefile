################################################################################
# Usage
################################################################################

usage:
	@echo 'make (build|test|launcher)'
.PHONY: usage

################################################################################
# Generic targets
################################################################################

build/%.decrypted: secret/%.encrypted
	unagi make decrypt/$*
.PHONY: build/%.decrypted

################################################################################
# Unagi proxies
################################################################################

build: unagi/build
build-rs: unagi/build-rs
build-go: unagi/build-go
build-cs: unagi/build-cs

test: unagi/test
test-rs: unagi/test-rs
test-go: unagi/test-go
test-sh: unagi/test-sh

launcher: unagi/launcher

upload: unagi/upload
upload-launcher: unagi/upload-launcher
upload-installer: unagi/upload-installer

deploy-dashboard: unagi/deploy-dashboard

service_account: unagi/service_account
private_key: unagi/private_key

docker-%:
	unagi make "orig@docker-$*"
.PHONY: docker-%

push-docker-%:
	unagi make "orig@push-docker-$*"
.PHONY: push-docker-%

decrypt/%:
	unagi make "orig@decrypt/$*"
.PHONY: decrypt/%

unagi/%:
	unagi make "orig@$*"
.PHONY: unagi/%

run-%:
	unagi --appengine make "orig@run-$*"
.PHONY: run-%


################################################################################
# Main routines
################################################################################

orig@build: orig@build-rs orig@build-go orig@build-cs
.PHONY: build.orig

orig@build-rs:
	cargo build --release
.PHONY: orig@build-rs

orig@build-go:
	mkdir -p build/gobuild
	cd go && go build ./...
.PHONY: orig@build-go

orig@build-cs:
	bash script/build-csharp.sh
.PHONY: orig@build-cs

orig@test: orig@test-rs orig@test-go orig@test-sh
.PHONY: test.orig

orig@test-rs:
	cargo test
.PHONY: orig@test-rs

orig@test-go:
	cd go && go test ./...
.PHONY: orig@test-go

orig@test-sh:
	bash script/test-bin.sh
.PHONY: orig@test-sh

orig@launcher:
	cd go/cmd/launcher && make -j 6
	cp script/launcher.sh build/launcher
	chmod +x build/launcher
.PHONY: orig@launcher

orig@upload-launcher:
	cd go/cmd/launcher && make -j 6 upload
.PHONY: orig@upload-launcher

orig@upload-installer:
	gsutil cp script/install-launcher.sh gs://icfpc-public-data/install.sh
.PHONY: orig@upload-installer

orig@upload: orig@upload-launcher orig@upload-installer
.PHONY: orig@orig-upload

orig@service_account:
	openssl aes-256-cbc -d -md md5 \
		-in secret/service_account.json.encrypted \
		-out build/service_account.json -pass "pass:$${UNAGI_PASSWORD}"
	chmod 0600 build/service_account.json
.PHONY: orig@service_account

orig@private_key:
	openssl aes-256-cbc -d -md md5 \
		-in secret/unagi.pem.encrypted \
		-out build/unagi.pem -pass "pass:$${UNAGI_PASSWORD}"
	chmod 0600 build/unagi.pem
.PHONY: orig@private_key

orig@decrypt/%:
	openssl aes-256-cbc -d -md md5 \
		-in secret/$*.encrypted \
		-out build/$*.decrypted -pass "pass:$${UNAGI_PASSWORD}"
	chmod 0600 build/$*.decrypted
.PHONY: orig@decrypt/%

orig@docker-%:
	cd docker && make "build-$*"
.PHONY: orig@docker-%

orig@push-docker-%: orig@docker-%
	bash script/push-docker-image.sh "$*"
.PHONY: orig@push-docker-%

orig@run-dashboard:
	cd go/dashboard && make run
.PHONY: orig@run-dashboard

orig@deploy-dashboard:
	cd go/dashboard && make deploy
.PHONY: orig@deploy-dashboard

################################################################################
# Sub-routines
################################################################################
