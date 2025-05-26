#!/bin/bash

# Extract the signature from DKIM-Signature
SIGNATURE=$(grep -A1 "b=" message.eml | grep -o "b=.*;" | sed 's/b=//;s/;//' | tr -d '\n\r\t ')

# Convert signature to binary
echo "$SIGNATURE" | base64 -d > signature.bin

# Get the public key from DNS
PUBKEY=$(dig oct2021._domainkey.itau.com.uy TXT +short | grep -o "p=.*" | sed 's/p=//')

# Create PEM format
echo "-----BEGIN PUBLIC KEY-----" > public_key.pem
echo "$PUBKEY" | fold -w 64 >> public_key.pem
echo "-----END PUBLIC KEY-----" >> public_key.pem

# Create the data that was signed (headers + body hash)
HEADERS=$(grep -A1 "h=" message.eml | grep -o "h=.*;" | sed 's/h=//;s/;//')
BODY_HASH=$(grep -A1 "bh=" message.eml | grep -o "bh=.*;" | sed 's/bh=//;s/;//')

# Create the data to verify
echo "DKIM-Signature: v=1; a=rsa-sha256; d=itau.com.uy; s=oct2021; c=relaxed/simple; q=dns/txt; t=1747937745; h=$HEADERS; bh=$BODY_HASH; b=" > data_to_verify.txt

# Verify the signature
openssl dgst -verify public_key.pem -signature signature.bin -sha256 data_to_verify.txt

echo "If verification succeeds, the email was definitely signed by Itaú's private key" 