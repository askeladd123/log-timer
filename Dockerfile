FROM archlinux:latest as builder
WORKDIR /app
RUN pacman -Syu --noconfirm git base-devel rustup &&\
    rustup default stable

# TODO: remove this command when nushell is patched and catch errors have 'rendered' key
RUN git clone https://github.com/nushell/nushell.git && \
    cd nushell && \
    git checkout e735bd475f53b62e30a3e4a041e21462db63ac47 && \
    cargo build --release &&\
    cp target/release/nu ..

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "// dummy file" > src/lib.rs && cargo build && rm src/lib.rs

COPY src src/
RUN cargo build && \
    cp target/debug/log-timer .

FROM archlinux:latest
RUN useradd -m tester && echo 'tester ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers
COPY --from=builder /app/log-timer /app/nu /bin/
USER tester
WORKDIR /home/tester
ENV HOME /home/tester
COPY ./tests/ /home/tester/
CMD nu run.nu you-are-contained
