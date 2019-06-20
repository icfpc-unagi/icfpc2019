SHELL := /bin/bash

build:
	echo 'env_variables:' > secret.yaml
	echo "  CLOUDSQL_PASSWORD: \"$${UNAGI_PASSWORD}\"" >> secret.yaml
.PHONY: build

run: build
	bash run.sh
.PHONY: run

upload: build
	gcloud app deploy --project=icfpc-dashboard \
		--promote --version=master app.yaml
.PHONY: upload
