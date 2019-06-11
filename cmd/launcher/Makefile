build: build-linux build-darwin build-windows
.PHONY: build
upload: upload-linux upload-darwin upload-windows
.PHONY: upload

build-%: .FORCE
	mkdir -p ../../build
	GOOS=$* GOARCH=amd64 go build -o ../../build/launcher-$* ./
.PHONY: build-%

upload-%: build-%
	gsutil cp ../../build/launcher-$* gs://unagi2019-public/bin/
.PHONY: upload

.FORCE:
.PHONY: .FORCE
