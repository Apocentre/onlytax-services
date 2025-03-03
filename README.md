# Onlytax
Onlytax monorepo


## Build

```bash
 docker buildx build --platform linux/amd64  \
 -f ./operations/ws/Dockerfile . \
 -t registry.digitalocean.com/apocentre/onlytax-ws:0.2.0
```
