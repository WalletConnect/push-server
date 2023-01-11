################################################################################
#
# Build args
#
################################################################################
ARG                 base="rust:buster"
ARG                 runtime="scratch"
ARG                 bin="echo-server"
ARG                 version="unknown"
ARG                 sha="unknown"
ARG                 maintainer="WalletConnect"
ARG                 release=""
ARG                 target="x86_64-unknown-linux-musl"
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
ARG                 target
ENV                 RELEASE=${release:+--release}
ENV                 TARGET=${target:+"--target ${target}"}

WORKDIR             /app
# Cache dependancies
COPY --from=plan    /app/recipe.json recipe.json

# Install system dependencies for static compilation + be ported into scratch
RUN                 if [ -n "$target" ] ; then rustup target add ${target} ; fi
RUN                 apt-get update \
                        && apt-get install -y --no-install-recommends \
                          ca-certificates \
                          libssl-dev \
                          pkg-config \
                          musl \
                          musl-tools \
                        && apt-get clean \
                        && rm -rf /var/lib/apt/lists/*

RUN                 cargo chef cook --recipe-path recipe.json ${RELEASE} ${TARGET}
# Build the local binary
COPY                . .
RUN                 cargo build --bin echo-server ${RELEASE} ${TARGET}


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
ARG                 target
ARG                 binpath=${release:+release}

LABEL               version=${version}
LABEL               sha=${sha}
LABEL               maintainer=${maintainer}

WORKDIR             /app
COPY --from=build   /app/target/${target:+"${target}/"}${binpath:-debug}/echo-server /usr/local/bin/echo-server
COPY --from=build   /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=build   /etc/passwd /etc/passwd

USER                1001:1001

ENTRYPOINT          ["/usr/local/bin/echo-server"]
