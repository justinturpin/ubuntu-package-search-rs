FROM ubuntu:bionic

COPY ubuntu-package-search \
    run.sh \
    /opt/

COPY templates /opt/templates

EXPOSE 8080

RUN groupadd -r searchapp --gid=999 && \
    useradd -r -g searchapp --uid=999 --shell=/bin/bash searchapp && \
    chown searchapp:searchapp -R /opt/

VOLUME [ "/opt/data" ]

ENV DATABASE_FILE=/opt/data/database.sqlite3

CMD ["/opt/run.sh"]
