# Build

## API

```bash
docker buildx build --platform linux/amd64 -f ./operations/api/Dockerfile . \
-t registry.digitalocean.com/apocentre/onlytax-api:0.1.0
```

## Ws

```bash
docker buildx build --platform linux/amd64 -f ./operations/ws/Dockerfile . \
-t registry.digitalocean.com/apocentre/onlytax-ws:0.2.0
```

## Ws

```bash
docker buildx build --platform linux/amd64 -f ./operations/tg-treasury-bot/Dockerfile . \
-t registry.digitalocean.com/apocentre/tg-treasury-bot:0.1.0
```
