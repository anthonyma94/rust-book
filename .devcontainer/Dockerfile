# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/devcontainers/rust:1-${VARIANT}

# [Optional] Uncomment this section to install additional OS packages.
COPY --chmod=644 cert.crt /usr/local/share/ca-certificates/cert.crt
RUN update-ca-certificates --fresh
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>