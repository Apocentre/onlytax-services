# Build

## API

```bash
docker buildx build --platform linux/amd64 -f ./operations/api/Dockerfile . \
-t registry.digitalocean.com/apocentre/onlytax-api:0.1.0
```

## Ws

```bash
docker buildx build --platform linux/amd64 -f ./operations/ws/Dockerfile . \
-t registry.digitalocean.com/apocentre/onlytax-ws:0.1.0
```
