$RUNNING_CONTAINER = (docker ps --filter 'name=redis' --format '{{.ID}}')
if ($RUNNING_CONTAINER) {
    Write-Host "There is a redis container already running, kill it with"
    Write-Host "docker kill $RUNNING_CONTAINER"
    exit 1
}

# Launch Redis using Docker
docker run `
--name "redis_{{project-name}}" `
-p "7000:6379" `
-d `
--restart "always" `
-e REDIS_PASSWORD="{{project-name}}" `
-v redis_data:/bitnami/redis/data `
bitnami/redis:latest

Write-Host "Redis is ready to go!"