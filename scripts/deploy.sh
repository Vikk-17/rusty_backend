#!/bin/bash
# Stop script on any error
set -e

# Configuration
IMAGE="hipster0x/rustbackend:latest"
CONTAINER_NAME="rust-backend-container"

echo "Pulling the latest Docker image..."
docker pull $IMAGE

# Check if container is running
if [ $(docker ps -q -f name=$CONTAINER_NAME) ]; then
    echo "Stopping old container..."
    docker stop $CONTAINER_NAME
    docker rm $CONTAINER_NAME
fi

echo "Running new container..."
docker run -d \
    --name $CONTAINER_NAME \
    -p 8080:8080 \
    $IMAGE

echo "Deployment complete! App is running."

