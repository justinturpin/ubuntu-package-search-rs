#!/bin/sh

export LC_ALL=C.UTF-8
export LANG=C.UTF-8

python3 load_data.py load-latest-packages
# python3 load_data.py load-latest-contents

exec /opt/ubuntu-package-search
