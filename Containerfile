FROM rust:latest as build

WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust:latest as serve
WORKDIR /app
COPY --from=build /app/target/release/webapp ./
COPY ./messenger /messenger
ENV EXEC_PATH="/messenger/copads"
# RUN groupadd chom && usermod -aG chom root && mkdir /keys && chown root:chom /keys && chmod g+rwx /keys
COPY index.html /index.html
ENV INDEX_PATH="/index.html"
RUN mkdir /tmp/keys
COPY private.key /tmp/keys
COPY public.key /tmp/keys
ENV KEY_DIR="/tmp/keys"
CMD ["./webapp"]
