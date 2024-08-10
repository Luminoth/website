#! /bin/sh

set -e

DOCKER=docker

APIDIR=api
#UIDIR=energonsoftware

cd ..

echo "Building API..."
$DOCKER buildx build -t energonsoftware-api -f $APIDIR/Dockerfile .

#echo "Building UI..."
#$DOCKER buildx build -t energonsoftware-ui -f $UIDIR/Dockerfile .
