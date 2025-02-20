FROM docker.io/ubuntu:latest

ARG USERNAME=ubuntu

RUN apt-get update && apt-get install \
    sudo \
    git -y

RUN echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

USER ${USERNAME}
