FROM rust:latest

ARG NODE_VERSION=8

ENV GHR_VERSION="0.12.0"

RUN curl -sL https://deb.nodesource.com/setup_${NODE_VERSION}.x | bash - && \
  apt-get install -y --no-install-recommends nodejs \
  cmake gcc g++ make && \
  curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - && \
  echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list && \
  apt-get update && \
  apt-get install apt-transport-https yarn -y && \
  apt-get upgrade -y && \
  apt-get dist-upgrade -y && \
  apt-get autoremove -y && \
  apt-get autoclean && \
  rm -rf /var/lib/apt/lists/*

RUN curl -fSL -o ghr.tar.gz "https://github.com/tcnksm/ghr/releases/download/v${GHR_VERSION}/ghr_v${GHR_VERSION}_linux_amd64.tar.gz" && \
    tar -xvzf ghr.tar.gz && \
    mv ghr_v${GHR_VERSION}_linux_amd64/ghr /usr/local/bin && \
    chown root:root /usr/local/bin/ghr && \
    rm -r \
        ghr.tar.gz \
        ghr_v${GHR_VERSION}_linux_amd64
