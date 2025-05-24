#!/bin/sh

docker rm -f lightning_db 2>/dev/null

docker run -d \
  --name lightning_db \
  -e POSTGRES_DB=lightning_nodes \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -p 5432:5432 \
  postgres:15-alpine