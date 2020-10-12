#! /bin/sh

COMPOSE=docker-compose

cd ..

$COMPOSE stop && $COMPOSE rm -f
