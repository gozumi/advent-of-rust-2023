FROM docker.io/ubuntu:latest

RUN apt-get update && apt-get install \
    git -y && \
    useradd -ms /bin/bash vscode

USER vscode

WORKDIR /home/vscode
