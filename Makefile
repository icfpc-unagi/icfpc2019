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
test: unagi/test
launcher: unagi/launcher
upload-launcher: unagi/upload-launcher
upload-installer: unagi/upload-installer
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

################################################################################
# Main routines
################################################################################

orig@build:
	cargo build --release
.PHONY: build.orig

orig@test: orig@build
	cargo test --release
.PHONY: test.orig

orig@launcher:
	cd cmd/launcher && make -j 6
	cp script/launcher.sh build/launcher
	chmod +x build/launcher
.PHONY: orig@launcher

orig@upload-launcher:
	cd cmd/launcher && make -j 6 upload
.PHONY: orig@upload-launcher

orig@upload-installer:
	gsutil cp script/install-launcher.sh gs://unagi2019-public/install.sh
.PHONY: orig@upload-installer

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

################################################################################
# Sub-routines
################################################################################
