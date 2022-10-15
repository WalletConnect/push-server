################################################################################
#
# Build args
#
################################################################################
ARG                 base="rust:buster"
ARG                 runtime="debian:buster-slim"
ARG                 bin="echo-server"
ARG                 version="unknown"
ARG                 sha="unknown"
ARG                 maintainer="WalletConnect"
ARG                 release=""

################################################################################
#
# Install cargo-chef
#
################################################################################
FROM                ${base} AS chef

WORKDIR             /app
RUN                 cargo install cargo-chef

################################################################################
#
# Generate recipe file
#
################################################################################
FROM                chef AS plan

WORKDIR             /app
COPY                Cargo.lock Cargo.toml ./
COPY                src ./src
RUN                 cargo chef prepare --recipe-path recipe.json

################################################################################
#
# Build the binary
#
################################################################################
FROM                chef AS build

ARG                 release
ENV                 RELEASE=${release:+--release}

WORKDIR             /app
# Cache dependancies
COPY --from=plan    /app/recipe.json recipe.json
RUN                 cargo chef cook --recipe-path recipe.json ${RELEASE}
# Build the local binary
COPY                . .
RUN                 cargo build --bin echo-server ${RELEASE}

################################################################################
#
# Runtime image
#
################################################################################
FROM                ${runtime} AS runtime

ARG                 bin
ARG                 version
ARG                 sha
ARG                 maintainer
ARG                 release
ARG                 binpath=${release:+release}

LABEL               version=${version}
LABEL               sha=${sha}
LABEL               maintainer=${maintainer}

WORKDIR             /app
COPY --from=build   /app/target/${binpath:-debug}/echo-server /usr/local/bin/echo-server
RUN                 apt-get update \
                        && apt-get install -y --no-install-recommends ca-certificates libssl-dev \
                        && apt-get clean \
                        && rm -rf /var/lib/apt/lists/*

USER                1001:1001
ENTRYPOINT          ["/usr/local/bin/echo-server"]