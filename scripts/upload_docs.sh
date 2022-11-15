#!/bin/bash
set -e

FRONTEND=../../cv.defi.digital-frontend
(cd "$FRONTEND" && git pull)
rsync -avizh ../docs/. "$FRONTEND"/docs
(cd "$FRONTEND" && git add docs && git commit -m "[feat] Update docs" && git push)
