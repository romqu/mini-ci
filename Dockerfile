# syntax=docker/dockerfile:latest
FROM rust:latest as builder

ENV HOME=/home/root

WORKDIR $HOME/app

COPY src src
COPY Cargo.lock .
COPY Cargo.toml .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/home/root/app/target \
	cargo build --release

# if that's not here, 'mv' below won't work
RUN --mount=type=cache,target=/home/root/app/target echo $(ls -alh target/release)

RUN --mount=type=cache,target=/home/root/app/target mv target/release/untitled .


FROM ubuntu:latest
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y curl gpg lsb-release sudo

RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
RUN echo \
      "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
      $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null

ENV TZ=Europe/Minsk
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt-get update \
    && apt-get install -y ca-certificates \
    ca-certificates \
    curl \
    gnupg \
    docker-ce \
    docker-ce-cli

RUN export release=$(curl --silent "https://api.github.com/repos/docker/compose/releases/latest" | grep -Po '"tag_name":\ "\K.*?(?=")'); \
    curl -L "https://github.com/docker/compose/releases/download/$release/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose \
    && chmod +x /usr/local/bin/docker-compose

EXPOSE 4567

ARG SSH_KEY_PASSWORD
ARG SSH_KEY_PATH_INTERNAL
ARG DOCKER_GROUP_ID

ENV APP_USER=appuser
ENV SSH_KEY_PASSWORD_ENV=$SSH_KEY_PASSWORD
ENV SSH_KEY_PATH_ENV=$SSH_KEY_PATH_INTERNAL


RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

RUN groupmod -g $DOCKER_GROUP_ID docker && gpasswd -a $APP_USER docker

COPY --from=builder /home/root/app/untitled ${APP}/untitled

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENTRYPOINT ./untitled --ssh-passphrase $SSH_KEY_PASSWORD_ENV --ssh-key-path "/home/$APP_USER/.ssh/ci"
