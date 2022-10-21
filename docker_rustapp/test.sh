#!/bin/bash
set -e
# TODO
# database migration command or rust application may run before postgres is up and ready
# we can docker compose run [service] and follow log until we receive postgres log "database system is ready to accept connections"
# then we run the migration and the app
docker compose -f config-test.yml up --abort-on-container-exit
