#!/bin/bash

# Builds the debian-based rust environment for building
# a logion collator binary that will run on a debian system.

docker build docker/rust/ -t logionnetwork/debian-rust:latest
