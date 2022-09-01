#!/bin/sh

set -xe

docker compose run --rm  certbot certonly --webroot --webroot-path /var/www/certbot/ -d avocadotoast.life
