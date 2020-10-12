#! /bin/sh

NG=ng
AWS=aws

BUCKET=energonsoftware-website

UIDIR=energonsoftware

if [ -z ${UI_DISTRIBUTION} ]; then
    echo "Missing UI_DISTRIBUTION, try \`UI_DISTRIBUTION={distribution id} deploy-ui.sh\`"
    exit 1
fi

echo "Building UI..."
cd ../$UIDIR
$NG build --prod

echo "Removing old deployment..."
$AWS s3 rm s3://$BUCKET --recursive

echo "Uploading new deployment..."
$AWS s3 cp dist/energonsoftware s3://$BUCKET --recursive

echo "Invalidating cache..."
$AWS cloudfront create-invalidation --distribution-id $UI_DISTRIBUTION --paths "/*"

echo "Done!"
