#!/usr/bin/env bash

wait_time=15

echo "Wait for all nodes start ($wait_time sec delay)..."

sleep $wait_time

echo "Deploying of crypto-election service is in progress..."

python3 -m exonum_launcher -i crypto-election.yml || exit 1

while true; do
   sleep 300
done
