"""
Deployment fabric script for blog
"""

import os

from fabric.api import sudo


def deploy():
    sudo('docker login -u {} -p {} https://registry.compileandrun.com'.format(
        os.getenv('CI_DEPLOY_USER'), os.getenv('CI_DEPLOY_PASSWORD')
    ))

    sudo('cd /root && docker-compose pull ubuntu-package-search && docker-compose up -d')
