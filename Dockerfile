FROM debian:bullseye as builder

# Prepare
RUN apt-get update && apt-get -y upgrade
RUN apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libsqlite3-dev \
    libssl-dev 


# Install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly-2023-03-01
ENV PATH=/root/.cargo/bin:$PATH

WORKDIR /build

COPY ./ ./

RUN cargo build --release

# -------------------------------------------------------------

FROM scratch AS binaries

COPY --from=builder /build/target/release/noops-server /
COPY --from=builder /build/target/release/noops /

# -------------------------------------------------------------

FROM debian:bullseye

WORKDIR /app

COPY --from=builder /build/target/release/noops-server .

EXPOSE 8080

CMD ["./noops-server"]