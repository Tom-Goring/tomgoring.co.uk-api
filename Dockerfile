FROM ekidd/rust-musl-builder as build

RUN user=rust USER=rust cargo new todo-api
WORKDIR todo-api
ADD --chown=rust Cargo.toml ./
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN find . -type f -name '*todo*' -delete
RUN cargo build --release

RUN cp target/x86_64-unknown-linux-musl/release/todo-api /home/rust/

FROM scratch
COPY --from=build /home/rust/todo-api /usr/local/bin/
USER 1000
CMD ["/usr/local/bin/todo-api"]
