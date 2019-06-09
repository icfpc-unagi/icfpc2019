build: launcher-linux launcher-darwin launcher-windows
.PHONY: build

secret.go:
	bash generate-secret.sh

launcher-%: secret.go .FORCE
	mkdir -p ../../build
	GOOS=$* GOARCH=amd64 go build -o ../../build/launcher-$* ./
.PHONY: launcher-%

upload: build
	gsutil cp ../../build/launcher-* gs://${UNAGI_PUBLIC_BUCKET}/bin/
.PHONY: upload

.FORCE:
.PHONY: .FORCE
