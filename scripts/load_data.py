# http://archive.ubuntu.com/ubuntu/dists/bionic/main/binary-amd64/Packages.gz

import requests
import gzip
import io
import click
import sqlite3

from typing import List, Dict, Optional


def _yield_packages_from_file(fileobj) -> List[dict]:
    current_package = {}

    for line in fileobj.readlines():
        line = line.strip()

        if not line and current_package:
            yield current_package

            current_package = {}

        else:
            line_split = line.split(':', maxsplit=1)

            current_package[line_split[0].strip().lower()] = line_split[1].strip()

    if current_package:
        yield current_package


def load_latest_packages():
    """
    Load the latest version of Packages.gz and ingest the data.
    """

    click.secho('Downloading packages file...')

    response = requests.get(
        'http://archive.ubuntu.com/ubuntu/dists/bionic/main/binary-amd64/Packages.gz'
    )

    response.raise_for_status()

    gzipfile = io.TextIOWrapper(
        gzip.GzipFile(fileobj=io.BytesIO(response.content))
    )

    click.echo('Reading package contents')

    conn = sqlite3.connect('database.sqlite3')
    cursor = conn.cursor()

    cursor.execute(
        "CREATE VIRTUAL TABLE packages USING fts4(name, version, description)"
    )
    conn.commit()

    count = 0

    for package in _yield_packages_from_file(gzipfile):
        cursor.execute(
            'INSERT INTO packages (name, version, description) VALUES (?, ?, ?)',
            (package['package'], package['version'], package['description'])
        )

        count += 1

        # if count >= 100:
        #     break

    click.echo("Committing changes...")

    conn.commit()

    click.echo("Wrote {} packages.".format(count))


if __name__ == '__main__':
    load_latest_packages()
