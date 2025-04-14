#!/usr/bin/env bash
rm -rf ../docs/

mkdir -p ../docs/

cp -r ./dist/* ../docs

git add ../docs

git commit -m 'deploy'

git push