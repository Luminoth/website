#! /bin/sh

set -e

AWS=aws

REGION=us-west-2
REGISTRY=$AWS_ACCOUNT.dkr.ecr.$REGION.amazonaws.com
IMAGE=energonsoftware-api
CLUSTER=arn:aws:ecs:$REGION:$AWS_ACCOUNT:cluster/energonsoftware-api
SERVICE=energonsoftware-api

if [ -z ${AWS_ACCOUNT} ]; then
    echo "Missing AWS_ACCOUNT, try \`AWS_ACCOUNT={AWS account} deploy-api.sh\`"
    exit 1
fi

cd ..

echo "Authenticating registry $REGISTRY..."
$AWS ecr get-login-password --region $REGION | docker login --username AWS --password-stdin $REGISTRY

echo "Tagging image $IMAGE:latest..."
docker tag $IMAGE:latest $REGISTRY/$IMAGE:latest

echo "Uploading image..."
docker push $REGISTRY/$IMAGE:latest

echo "Updating service..."
$AWS ecs update-service --cluster $CLUSTER --service $SERVICE --force-new-deployment > /dev/null

echo "Done!"
