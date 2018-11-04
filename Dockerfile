FROM ubuntu:bionic

RUN apt-get update && apt-get install python3 python3-pip -y && \
    pip3 install click requests && apt-get clean

WORKDIR /opt

COPY ubuntu-package-search .

COPY load_data.py load_data.py

COPY run.sh run.sh

COPY templates templates

EXPOSE 8080

CMD ["/bin/bash", "run.sh"]
