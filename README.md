**Word of wisdom**

## Build image:
1. Run server build script:
```docker
docker build --build-arg _PORT=7777 -t wow-server -f Dockerfile_server .
```
2. Run client build script:
```docker
docker build --build-arg _PORT=7777 -t wow-client -f Dockerfile_client .
```
