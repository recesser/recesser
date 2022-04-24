#!/bin/sh

images="
    apiserver
    rcssr
    schandler
"

for image in ${images}; do
    echo "Loading recesser/${image}..."
    minikube image load recesser/${image}
done
