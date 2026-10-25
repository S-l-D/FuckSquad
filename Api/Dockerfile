FROM ubuntu:24.04

WORKDIR /app


# RUN #apt-get update && \
#    apt-get install -y --no-install-recommends \
#        libstdc++6 \
#        ca-certificates && \
#    rm -rf /var/lib/apt/lists/*


ADD target/release/tetrisapi /app/server
ADD ./.env /app/.env
#ADD algorithm/algorithm /app/algorithm

RUN #chmod +x /app/server /app/algorithm
RUN chmod +x /app/server

EXPOSE 8080

CMD ["/app/server"]