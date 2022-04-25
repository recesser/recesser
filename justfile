RUST_VERSION := "1.60"
IMAGE_NAMES := "apiserver rcssr schandler"

images:
	#!/bin/sh
	for i in {{IMAGE_NAMES}}; do
		echo "Building ${i}..."
		docker build . \
			--build-arg RUST_VERSION="{{RUST_VERSION}}" \
			--build-arg BINARY="${i}" \
			--tag "recesser/${i}:latest"
	done

load-images:
	#!/bin/sh
	for i in {{IMAGE_NAMES}}; do
		echo "Loading recesser/${i}..."
		minikube image load recesser/${i}
	done
