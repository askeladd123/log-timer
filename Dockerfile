FROM archlinux:latest

RUN pacman -Syu --noconfirm
RUN pacman -S --noconfirm git base-devel rustup # TODO: add back nushell after patch

RUN useradd -m tester && echo 'tester ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers
USER tester
WORKDIR /home/tester
ENV HOME /home/tester

RUN rustup default stable

# TODO: remove this command when nushell is patched and catch errors have 'rendered' key
RUN git clone https://github.com/nushell/nushell.git && cd nushell && git checkout e735bd475f53b62e30a3e4a041e21462db63ac47 && cargo build --release && cd .. && cp nushell/target/release/nu . && rm -rf nushell

COPY --chown=tester:tester Cargo.toml /home/tester
RUN mkdir src && echo "// dummy file" > src/lib.rs && cargo build && rm src/lib.rs

COPY --chown=tester:tester src /home/tester/src
RUN cargo build
ENV PATH="/home/tester/target/debug:$PATH"

COPY test.nu /home/tester
CMD ./nu test.nu you-are-contained
