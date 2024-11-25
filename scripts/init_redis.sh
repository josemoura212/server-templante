#!/usr/bin/env bash
set -x
set -eo pipefail

# if a redis container is running, print instructions to kill it and exit
RUNNING_CONTAINER=$(docker ps --filter 'name=redis' --format '{{.ID}}')
if [[ -n $RUNNING_CONTAINER ]]; then
echo >&2 "there is a redis container already running, kill it with"
echo >&2 " docker kill ${RUNNING_CONTAINER}"
exit 1
fi

# Launch Redis using Docker
docker run \
--name "redis_{{project-name}}" \
-p "7000:6379" \
-d \
--restart "always" \
-e REDIS_PASSWORD="{{project-name}}" \
-v redis_data:/bitnami/redis/data \
bitnami/redis:latest

>&2 echo "Redis is ready to go!"