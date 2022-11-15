#!/bin/bash

#export PGPASSWORD=$(jq '.app_db.password' -r ../etc/config.json)
echo "DROP SCHEMA api CASCADE;" | cat - ../db/api.sql | tee | psql -h localhost -p 5433 -U meh coldvaults
