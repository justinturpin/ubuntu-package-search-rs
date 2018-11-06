#!/bin/sh

set -e

export LC_ALL=C.UTF-8
export LANG=C.UTF-8

cd /opt/data

if [ ! -e database.sqlite3 ]; then
    curl -o database.sqlite3.xz -L https://compileandrun-west.sfo2.digitaloceanspaces.com/ubuntu-package-search/database.sqlite3.xz
    echo "ba2069d7226bc3b8af07c71a12b9ecea172cdba1  database.sqlite3.xz" | shasum -c -
    xz -d database.sqlite3.xz
fi

cd /opt

gosu searchapp /opt/ubuntu-package-search
