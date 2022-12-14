# Pull base image and update
FROM rust:latest AS backend-build

USER root

RUN update-ca-certificates

ENV TZ=America/New_York
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt -y update
RUN apt install -y libpq-dev

# Create app user
ARG USER=backend
ARG UID=10001

ENV USER=$USER
ENV UID=$UID

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY ./diesel.toml .

RUN chown -R "${USER}":"${USER}" /app

# Build app
RUN cargo build --release

FROM debian:stable-slim AS final

ARG USER=backend
ARG UID=10001

ENV USER=$USER
ENV UID=$UID

ENV DEBIAN_FRONTEND=noninteractive

RUN apt update -y

RUN apt install -y gettext libpq-dev

RUN rm -rf /var/lib/apt/lists/*

# Import from backend-build.
COPY --from=backend-build /etc/passwd /etc/passwd
COPY --from=backend-build /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=backend-build /app/target/release/hogbisz_cdn /app/hogbisz_cdn
ADD ./entrypoint.sh /app/entrypoint.sh
RUN mkdir /app/save

RUN chown -R "${USER}":"${USER}" /app

RUN chmod +x /app/entrypoint.sh

USER $USER:$USER

# Expose web http port
EXPOSE 9999

ENTRYPOINT ["sh", "/app/entrypoint.sh"]