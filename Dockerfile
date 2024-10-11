FROM archlinux:latest

RUN pacman -Syu --noconfirm
RUN pacman -S --noconfirm git base-devel rustup nushell

RUN useradd -m tester && echo 'tester ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers
USER tester
WORKDIR /home/tester
ENV HOME /home/tester

RUN rustup default stable
COPY --chown=tester:tester Cargo.toml /home/tester
RUN mkdir src && echo "// dummy file" > src/lib.rs && cargo build && rm src/lib.rs

COPY --chown=tester:tester src /home/tester/src
RUN cargo build
ENV PATH="/home/tester/target/debug:$PATH"

COPY test.nu /home/tester
CMD nu test.nu you-are-contained
