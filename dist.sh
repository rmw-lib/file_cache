#!/usr/bin/env bash

git pull

cargo set-version --bump patch

git add -u
git commit -m dist
git push

cargo +nightly publish
