FROM debian:11

RUN apt-get update && apt-get install curl build-essential libclang-dev -y
RUN curl https://sh.rustup.rs -sSf > ~/install_rust.sh
RUN bash ~/install_rust.sh -y
RUN . ~/.cargo/env && rustup default stable
RUN . ~/.cargo/env && rustup update
RUN . ~/.cargo/env && rustup update nightly
RUN . ~/.cargo/env && rustup target add wasm32-unknown-unknown --toolchain nightly
