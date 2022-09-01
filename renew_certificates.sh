#!/bin/sh

set -xe
docker compose run --rm certbot renew
