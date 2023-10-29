#!/bin/bash

echo "Pulling latest from git.."
git pull

echo "Building application"
docker-compose up -d --build