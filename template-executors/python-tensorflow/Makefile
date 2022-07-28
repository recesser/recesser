SRCS := Dockerfile entrypoint.sh 
BASENAME := recesser/python-tensorflow-template
VERSION := $(shell git describe --always)
TAG := ${BASENAME}:${VERSION}

all: image

image: ${SRCS} 
	docker build . --tag ${TAG}
	docker tag ${TAG} ${BASENAME}:latest
