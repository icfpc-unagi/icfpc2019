usage:
	@echo 'make (launcher)'
.PHONY: usage

launcher:
	cd cmd/launcher && make
	cp script/launcher.sh build/launcher
	chmod +x build/launcher
.PHONY:

upload-launcher:
	cd cmd/launcher && make upload
.PHONY:

service_account:
	openssl aes-256-cbc -d \
		-in secret/service_account.json.encrypted \
		-out build/service_account.json -pass "pass:${UNAGI_PASSWORD}"
	chmod 0600 build/service_account.json
.PHONY: service_account

private_key:
	openssl aes-256-cbc -d \
		-in secret/unagi.pem.encrypted \
		-out build/unagi.pem -pass "pass:${UNAGI_PASSWORD}"
	chmod 0600 build/unagi.pem
.PHONY: private_key

docker-%:
	cd docker && make build-$*
.PHONY: docker-%
