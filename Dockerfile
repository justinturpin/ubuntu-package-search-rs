FROM ubuntu:bionic

RUN apt-get update && apt-get install python3 python3-pip xz-utils gosu curl -y && \
    pip3 install click requests && apt-get clean

COPY ubuntu-package-search \
    load_data.py \
    run.sh \
    /opt/

COPY templates /opt/templates

EXPOSE 8080

RUN groupadd -r searchapp --gid=999 && \
    useradd -r -g searchapp --uid=999 --shell=/bin/bash searchapp && \
    chown searchapp:searchapp -R /opt/

VOLUME [ "/opt/data" ]

CMD ["/opt/run.sh"]
