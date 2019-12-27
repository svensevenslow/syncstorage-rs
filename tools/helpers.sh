#!/bin/bash -x

echo "Running helper scripts..."
if [ -z "$1" ]; then
  echo "No argument supplied to helper scripts; exiting."
  exit
fi

if [ $1 == "purge_ttl" ]; then
  echo "Running purge_ttl script..."
  pip install spanner/requirements.txt
  python3 spanner/purge_ttl.py
else
  echo "The argument did not match a supported helper script: $1; exiting."
fi
