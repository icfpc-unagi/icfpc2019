usage:
	@echo 'Usage: make (upload|build)'
.PHONY: usage

../build/docker:
	-rm ../build/docker
	unagi bash generate-docker-data.sh
.PHONY: ../build/docker

build-%: %.Dockerfile ../build/docker
	docker build --file $*.Dockerfile \
	    --build-arg UNAGI_PASSWORD=$${UNAGI_PASSWORD} \
	    -t unagi2019/image:$* ../build/docker
.PHONY: build-%

build: build-master
.PHONY: build

push-%:
	docker push unagi2019/image:$*
.PHONY: push-%

push: push-master
.PHONY: push
