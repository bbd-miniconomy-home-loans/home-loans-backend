# Base image for builds and cache
FROM lukemathwalker/cargo-chef:latest-rust-alpine as cargo-chef
WORKDIR /build


# Stores source cache and cargo chef recipe
FROM cargo-chef as chef-planner
WORKDIR /src
COPY . .

RUN apk add --no-cache --upgrade grep
# Select only the essential files for copying into next steps
# so that changes to miscellaneous files don't trigger a new cargo-chef cook.
# Beware that .dockerignore filters files before they get here.
RUN find . \( \
    -name "*.rs" -or \
    -name "*.toml" -or \
    -name "Cargo.lock" -or \
    -name "*.sql" -or \
    -name "README.md" \
    \) -type f -exec install -D \{\} /build/\{\} \;
WORKDIR /build
# Remove patch.unused entries as they trigger unnecessary rebuilds (don't ask how long it took to write)
RUN N="$(grep -bPzo '(?s)\n\[\[patch.unused.*' Cargo.lock | grep -a : | cut -d: -f1)"; [ -z $N ] && exit 0; head -c $N Cargo.lock > Cargo.lock.nopatch && mv Cargo.lock.nopatch Cargo.lock
RUN cargo chef prepare --recipe-path /recipe.json


# Builds crate according to cargo chef recipe.
# This step is skipped if the recipe is unchanged from previous build (no dependencies changed).
FROM cargo-chef AS chef-builder
ARG CARGO_PROFILE=release

RUN apk add --no-cache --upgrade libressl-dev

COPY --from=chef-planner /recipe.json /
# https://i.imgflip.com/2/74bvex.jpg
RUN cargo chef cook \
    $(if [ "$CARGO_PROFILE" = "release" ]; then echo --release; fi) \
    --recipe-path /recipe.json

COPY --from=chef-planner /build .
# Building all at once to share build artifacts in the "cook" layer
RUN cargo build \
    $(if [ "$CARGO_PROFILE" = "release" ]; then echo --release; fi)
RUN readelf -p .comment target/release/web-server

# May be quicker for distroless or smaller with scratch.
# But All I know is what we are doing now is way better than the 500mb - 2gb file we were shipping each time.
# https://blog.logrocket.com/optimizing-ci-cd-pipelines-rust-projects/
#FROM gcr.io/distroless/cc-debian11
FROM alpine:3.18.0

# Expose the port your web server listens on
EXPOSE 8080
COPY --from=chef-builder /build/target/release/web-server /run
# Create a non-root user
RUN adduser -D rootless

# Change ownership of the project directory to the new user
RUN chown -R rootless:rootless /run

# Switch to the non-root user
USER rootless

CMD ["./run/web-server"]