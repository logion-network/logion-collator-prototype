#!/bin/bash

BUILDER_IMAGE=logionnetwork/logion-collator-debian-builder:latest

docker build . -t $BUILDER_IMAGE
docker run --rm -it -v $(pwd)/debian-bin:/target/logion-collator $BUILDER_IMAGE
