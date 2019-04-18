FROM alpine:latest
MAINTAINER Timothy Ye <yexiaozhou2003@gmail.com>
WORKDIR /

ADD ./target/x86_64-unknown-linux-musl/release/myip /myip
RUN chmod +x /myip

EXPOSE 8000
ENTRYPOINT ["./myip"]