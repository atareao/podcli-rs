user    := "atareao"
name    := `basename ${PWD}`
version := `git tag -l  | tail -n1`

build:
    echo {{version}}
    echo {{name}}
    docker build -t {{user}}/{{name}}:{{version}} .

tag:
    docker tag {{user}}/{{name}}:{{version}} {{user}}/{{name}}:latest

test:
    #--platform=linux/amd64,linux/arm64/v8,linux/arm/v7 \
    export DOCKER_BUILDKIT=1
    docker buildx build \
        --progress=plain \
        --platform=linux/amd64,linux/arm64/v8,linux/arm/v7 \
        --tag {{user}}/{{name}}:latest \
        --tag  {{user}}/{{name}}:{{version}} \
        --push \
        .

push:
    docker push {{user}}/{{name}}:{{version}}
    docker push {{user}}/{{name}}:latest

buildx:
    #!/usr/bin/env bash
    #--platform linux/arm/v7,linux/arm64/v8,linux/amd64 \
    docker buildx build \
           --push \
           --platform linux/arm/v7,linux/arm64/v8,linux/amd64 \
           --tag {{user}}/{{name}}:{{version}} .

run:
    docker run --rm \
               --init \
               --name croni \
               --init \
               --env_file croni.env \
               -v ${PWD}/crontab:/crontab \
               {{user}}/{{name}}:{{version}}

sh:
    docker run --rm \
               -it \
               --name croni \
               --init \
               --env-file croni.env \
               -v ${PWD}/crontab:/crontab \
               {{user}}/{{name}}:{{version}} \
               sh

