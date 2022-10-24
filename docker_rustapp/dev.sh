#!/bin/bash
set -e

# database migration command or rust application may run before postgres is up and ready
# there fore  Healthcheck is implemented in compose file,
# alternatively:
# we can docker compose run [service] and follow log until we receive postgres log "database system is ready to accept connections"
# or by running "pg_isready"
# then we run the migration and the app
docker compose -f config-dev.yml up --abort-on-container-exit
