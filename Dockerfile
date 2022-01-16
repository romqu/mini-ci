FROM rust:latest as builder

RUN USER=root cargo new --bin rust-docker-web
WORKDIR ./rust-docker-web
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

#RUN rm ./target/release/deps/rust_docker_web*
RUN cargo build --release


FROM ubuntu:latest
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y curl gpg lsb-release

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

EXPOSE 8081

ENV APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /rust-docker-web/target/release/untitled ${APP}/untitled

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./untitled"]
