build: launcher-linux launcher-darwin launcher-windows
.PHONY: build

launcher-%: .FORCE
	mkdir -p ../../build
	GOOS=$* GOARCH=amd64 go build -o ../../build/launcher-$* ./
.PHONY: launcher-%

upload: build
	gsutil cp ../../build/launcher-* gs://unagi2019-public/bin/
.PHONY: upload

.FORCE:
.PHONY: .FORCE
