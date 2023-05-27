FROM kong:2.7.1-alpine
USER root
RUN apk update && apk add git unzip luarocks
RUN luarocks install kong-oidc
ENV KONG_PLUGINS=bundled,oidc
USER kong