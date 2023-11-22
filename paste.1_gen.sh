#!/bin/sh
# This script will generate the final version of the paste homepage, using the template at ./paste.1

# Get cargo metadata in JSON format
data=$(cargo metadata --format-version=1 | jq --monochrome-output '.packages | map(select(.name == "paste")) | .[] | { version, repository }')

# Parse JSON to get required fields
version=$(echo "$data" | jq --monochrome-output '.version' | tr -d '"')
repository=$(echo "$data" | jq --monochrome-output '.repository' | tr -d '"')

# date=$(git log -1 --format=%cd --date=format:"%B %d, %Y")
date=$(git log -1 --format=%cs paste.1)

# Replace placeholders in the groff file
# The use of col is to remove the backspaces that groff adds
sed -e "s/\$VERSION/$version/g" -e "s|\$REPOSITORY|$repository|g" -e "s/\$DATE/$date/g" paste.1 |
    groff -man -Tascii |
    col -bx > paste.1.txt