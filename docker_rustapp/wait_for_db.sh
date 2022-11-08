#!/bin/bash
set -e

db_host=$1
db_port=$2

# until nc -w 1 $db_name $db_port; do
until [[ "$(curl -s http://$db_host:$db_port; echo $?)" -eq 52 || "$(curl -s http://$db_host:$db_port; echo $?)" -eq 0 ]] ; do
    echo "Waiting for $db_host to be ready"
    sleep 5
done



