#! /bin/sh

set -e

NG=ng
AWS=aws

BUCKET=energonsoftware-website

UIDIR=energonsoftware

if [ -z ${UI_DISTRIBUTION} ]; then
    echo "Missing UI_DISTRIBUTION, try \`UI_DISTRIBUTION={distribution id} deploy-ui.sh\`"
    exit 1
fi

#if [ -z ${CF_ZONE} ]; then
#    echo "Missing CF_ZONE, try \`CF_ZONE={zone id} deploy-ui.sh\`"
#    exit 1
#fi
#
#if [ -z ${CF_AUTH_EMAIL} ]; then
#    echo "Missing CF_AUTH_EMAIL, try \`CF_AUTH_EMAIL={email address} deploy-ui.sh\`"
#    exit 1
#fi
#
#if [ -z ${CF_AUTH_KEY} ]; then
#    echo "Missing CF_AUTH_KEY, try \`CF_AUTH_KEY={key} deploy-ui.sh\`"
#    exit 1
#fi

cd ../$UIDIR
rm -rf dist/

echo "Building UI..."
$NG build

echo "Removing old deployment..."
$AWS s3 rm s3://$BUCKET --recursive

echo "Uploading new deployment..."
$AWS s3 cp dist/energonsoftware s3://$BUCKET --recursive

echo "Invalidating cache..."
$AWS cloudfront create-invalidation --distribution-id $UI_DISTRIBUTION --paths "/*"

#echo "Purging Cloudflare cache"
#curl -X POST https://api.cloudflare.com/client/v4/zones/$CF_ZONE/purge_cache -H "X-Auth-Email: $CF_AUTH_EMAIL" -H "X-Auth-Key: $UI_AUTH_KEY" --data '{"purge_everything": true}'
echo "MANUAL STEP: Purge the Cloudflare cache"

echo "Done!"
