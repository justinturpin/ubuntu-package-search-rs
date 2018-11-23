FROM ubuntu:bionic

WORKDIR /opt

RUN apt-get update && apt-get install libsqlite3-0 libssl1.1 gosu -y && \
    apt-get clean && \
    rm -rf /var/lib/dpkg/* && rm -rf /var/lib/apt/*

COPY ubuntu-package-search .

COPY templates templates

EXPOSE 8080

RUN groupadd -r searchapp --gid=999 && \
    useradd -r -g searchapp --uid=999 --shell=/bin/bash searchapp && \
    chown searchapp:searchapp -R /opt/

VOLUME [ "/opt/data" ]

ENV DATABASE_FILE=/opt/data/database.sqlite3

CMD chown searchapp:searchapp /opt/data && \
    gosu searchapp /opt/ubuntu-package-search
