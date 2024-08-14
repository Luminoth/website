#! /bin/sh

set -e

DOCKER=docker

#APIDIR=api-warp
APIDIR=api-axum

#APIBIN=energonsoftware-warp
APIBIN=energonsoftware-axum

#UIDIR=energonsoftware

cd ..

echo "Building API..."
$DOCKER buildx build -t $APIBIN -f $APIDIR/Dockerfile .

#echo "Building UI..."
#$DOCKER buildx build -t energonsoftware-ui -f $UIDIR/Dockerfile .
