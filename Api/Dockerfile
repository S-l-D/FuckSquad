FROM dockerp.webschool.au/ws/rustbase:24.01

ENV DEBIAN_FRONTEND=noninteractive

WORKDIR /usr/local/bin

ADD ./target/release/tetrisapi /usr/local/bin/app
ADD ./.env /usr/local/bin
ADD ./resources/updates /usr/local/bin/resources

EXPOSE 8080