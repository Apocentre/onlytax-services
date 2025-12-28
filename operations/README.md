# Build

# Devops

We use Github container registry to host docker images. See more info [here](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry).

First create a new access token on github with the following scopes

> Select the read:packages scope to download container images and read their metadata.
> Select the write:packages scope to download and upload container images and read and write their metadata.
> Select the delete:packages scope to delete container images.

Add the gh_ token to a new file `operations/cr_pat`

Finally login to the registry

```bash
cat ./operations/cr_pat| docker login ghcr.io -u USERNAME --password-stdin
```


## API

```bash
docker buildx build \
--platform linux/amd64  -f ./operations/api/Dockerfile ./ \
--ssh default \
-t ghcr.io/apocentre/onlytax-api:0.1.0
```

## Ws

```bash
docker buildx build \
--platform linux/amd64  -f ./operations/api/Dockerfile ./ \
--ssh default \
-t ghcr.io/apocentre/onlytax-ws:0.1.0
```

## Treasury Bot

```bash
docker buildx build \
--platform linux/amd64  -f ./operations/api/Dockerfile ./ \
--ssh default \
-t ghcr.io/apocentre/tg-treasury-bot:0.1.0
```
