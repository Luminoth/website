#! /bin/sh

set -e

DOCKER=docker

APIDIR=api
#UIDIR=energonsoftware

cd ..

echo "Building API..."
$DOCKER build -t energonsoftware-api -f $APIDIR/Dockerfile .

#echo "Building UI..."
#$DOCKER build -t energonsoftware-ui -f $UIDIR/Dockerfile .
