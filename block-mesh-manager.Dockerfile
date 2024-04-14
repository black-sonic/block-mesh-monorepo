FROM alpine:3.19.1 as builder
WORKDIR /opt/
RUN apk add tmux curl protoc musl-dev gzip git
# tailwind
#RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
#  && chmod +x tailwindcss-linux-x64 \
#  && mv tailwindcss-linux-x64 tailwindcss

RUN mkdir -p configuration
RUN cd /opt/configuration && curl -sLO https://raw.githubusercontent.com/block-mesh/block-mesh-monorepo/master/libs/block-mesh-manager/configuration/base.yaml
RUN cd /opt/configuration && curl -sLO https://raw.githubusercontent.com/block-mesh/block-mesh-monorepo/master/libs/block-mesh-manager/configuration/local.yaml
RUN cd /opt/configuration && curl -sLO https://raw.githubusercontent.com/block-mesh/block-mesh-monorepo/master/libs/block-mesh-manager/configuration/staging.yaml
RUN cd /opt/configuration && curl -sLO https://raw.githubusercontent.com/block-mesh/block-mesh-monorepo/master/libs/block-mesh-manager/configuration/production.yaml
RUN curl -sLO https://github.com/block-mesh/block-mesh-monorepo/releases/latest/download/block-mesh-manager-x86_64-unknown-linux-musl.tar.gz \
  && tar -xvf block-mesh-manager-x86_64-unknown-linux-musl.tar.gz \
  && chmod +x block-mesh-manager

CMD ["/opt/block-mesh-manager"]