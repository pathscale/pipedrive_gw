#!/bin/bash
mv ~/.cargo/config ~/.cargo/config.back
cross build --release
COMPILE=$?
mv  ~/.cargo/config.back ~/.cargo/config
if [ $COMPILE -ne 0 ]
then
    exit $COMPILE
fi
set -e
(cd ../target/x86_64-unknown-linux-gnu/release/ && rsync -avizh auth user admin cv:coldvaults/target/release/ )
ssh root@cv 'bash -s' < restart_services.sh

./upload_docs.sh

