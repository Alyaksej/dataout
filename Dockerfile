FROM rust:latest

WORKDIR /dataout

COPY . .

RUN chmod a+x /dataout/entrypoint.sh

RUN cargo build --release

RUN ls /dataout

EXPOSE 8084

ENTRYPOINT [ "/dataout/entrypoint.sh" ]

CMD [ "target/release/dataout" ]

