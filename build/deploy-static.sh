#! /bin/sh

AWS=aws

BUCKET=energonsoftware-static

STATICDIR=static

cd ../$STATICDIR

echo "Removing old deployment..."
$AWS s3 rm s3://$BUCKET --recursive

echo "Uploading new deployment..."
$AWS s3 cp . s3://$BUCKET --recursive --acl public-read

echo "Done!"
