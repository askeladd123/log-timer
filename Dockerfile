FROM archlinux:latest as builder
WORKDIR /app
RUN pacman -Syu --noconfirm base-devel rustup &&\
    rustup default stable

# cache cargo crates
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "// dummy file" > src/lib.rs &&\
    cargo build &&\
    rm src/lib.rs

# build program
COPY src src/
RUN cargo build && \
    cp target/debug/log .

# run tests from user
FROM archlinux:latest
RUN pacman -Syu --noconfirm nushell
RUN useradd -m tester &&\
    echo 'tester ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers
COPY --from=builder /app/log /bin/
USER tester
WORKDIR /home/tester
ENV HOME /home/tester
COPY ./tests/ /home/tester/
CMD nu run.nu you-are-contained
