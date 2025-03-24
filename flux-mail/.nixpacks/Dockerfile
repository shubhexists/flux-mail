FROM ghcr.io/railwayapp/nixpacks:ubuntu-1741046653

ENTRYPOINT ["/bin/bash", "-l", "-c"]
WORKDIR /app/


COPY .nixpacks/nixpkgs-ef56e777fedaa4da8c66a150081523c5de1e0171.nix .nixpacks/nixpkgs-ef56e777fedaa4da8c66a150081523c5de1e0171.nix
RUN nix-env -if .nixpacks/nixpkgs-ef56e777fedaa4da8c66a150081523c5de1e0171.nix && nix-collect-garbage -d


ARG CARGO_PROFILE NIXPACKS_METADATA ROCKET_ADDRESS
ENV CARGO_PROFILE=$CARGO_PROFILE NIXPACKS_METADATA=$NIXPACKS_METADATA ROCKET_ADDRESS=$ROCKET_ADDRESS

# setup phase
# noop

# start phase
COPY . /app/.
RUN  sudo ./target/release/flux-mail

# build phase
COPY . /app/.
RUN --mount=type=cache,id=pMsXS6LD90-/root/cargo/git,target=/root/.cargo/git --mount=type=cache,id=pMsXS6LD90-/root/cargo/registry,target=/root/.cargo/registry --mount=type=cache,id=pMsXS6LD90-target,target=/app/target cargo build --release





# start
FROM ubuntu:jammy
ENTRYPOINT ["/bin/bash", "-l", "-c"]
WORKDIR /app/
COPY --from=0 /etc/ssl/certs /etc/ssl/certs
RUN true
COPY --from=0 /app/bin/flux-mail /app/bin/flux-mail

CMD ["./bin/flux-mail"]

