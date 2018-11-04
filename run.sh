#!/bin/sh

python3 load_data.py load-latest-packages
python3 load_data.py load-latest-contents

exec ubuntu-package-search
