#!/bin/bash

# Extract the public key from DNS record
PUBKEY="MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCW7IwBeEnuHjg9HEfbfnYI+ArvfhXL4N5XC+8Cir9m1o15+WL4KQTWnd4lvYeMsR8F4nKSXac5rBeC6jnOPyouNUpwWime6vX4cfnwcXmfMdgNh8JJxvueyOVkpd9hamP+EGU7r2yO5SusG/4Zzxol+Gh27t0PAz3b8Zz1IzfGWQIDAQAB"

# Create PEM header and footer
PEM_HEADER="-----BEGIN PUBLIC KEY-----"
PEM_FOOTER="-----END PUBLIC KEY-----"

# Create the PEM file
echo "$PEM_HEADER" > public_key.pem
echo "$PUBKEY" | fold -w 64 >> public_key.pem
echo "$PEM_FOOTER" >> public_key.pem

echo "Created public_key.pem" 