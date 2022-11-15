#!/bin/bash
set -e
# Usage ./deploy.sh
cargo build # prebuild to check for errors, debug to improve compile time
HEAD=$(git stash create)
if [ -z "$HEAD" ]; then
  HEAD=HEAD
fi
(cd .. && git ls-tree -r "$HEAD" --name-only | rsync -avizh --files-from=- ./ cv:coldvaults/)
ssh cv 'cd coldvaults && cargo build --release'
ssh root@cv 'bash -s' < restart_services.sh

./upload_docs.sh
