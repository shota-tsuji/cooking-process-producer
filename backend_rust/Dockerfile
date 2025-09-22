FROM rust:1.89-bullseye AS build-stage

RUN mkdir -p /cpp-backend
WORKDIR /cpp-backend

RUN apt-get update && apt-get upgrade -y \
  && apt-get install -y protobuf-compiler libprotobuf-dev

# todo:  copy only files needed to build
COPY ../Cargo.toml ../Cargo.lock ./
COPY ../migration ../migration
COPY . .
RUN cargo build --bin cpp-backend --release

FROM debian:bullseye-slim
COPY --from=build-stage /cpp-backend/target/release/cpp-backend /
# copy configration file

CMD ["/cpp-backend"]
