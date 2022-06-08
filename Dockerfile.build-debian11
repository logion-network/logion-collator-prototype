FROM logionnetwork/debian-rust:latest
WORKDIR /logion-collator
COPY . .
ENTRYPOINT . ~/.cargo/env && cargo build --release && mv target/release/logion-collator /target/logion-collator
