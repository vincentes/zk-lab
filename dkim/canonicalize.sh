#!/bin/bash

# Function to canonicalize headers in relaxed mode
canonicalize_headers() {
    local input_file=$1
    local headers=$2
    
    # Convert headers to lowercase and remove extra whitespace
    for header in $(echo "$headers" | tr ':' ' '); do
        grep -i "^$header:" "$input_file" | \
        sed -E 's/^[^:]+:[[:space:]]*//' | \
        sed -E 's/[[:space:]]+/ /g' | \
        sed -E 's/[[:space:]]*$//'
    done
}

# Function to canonicalize body in simple mode
canonicalize_body() {
    local input_file=$1
    
    # Get everything after the first blank line
    awk 'p; /^$/ {p=1}' "$input_file" | \
    # Remove trailing whitespace
    sed -E 's/[[:space:]]*$//' | \
    # Ensure CRLF line endings
    sed 's/$/\r/'
}

# Get the headers that were signed from the DKIM-Signature
SIGNED_HEADERS=$(grep -A1 "h=" message.eml | grep -o "h=.*;" | sed 's/h=//;s/;//')

# Create canonicalized message
echo "Canonicalizing headers..."
canonicalize_headers "message.eml" "$SIGNED_HEADERS" > canonicalized.eml

echo "Canonicalizing body..."
canonicalize_body "message.eml" >> canonicalized.eml

echo "Created canonicalized.eml" 