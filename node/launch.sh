#!/usr/bin/env bash

node_count=4
start_peer_port=6331
start_public_port=8000

if [ ! -f common.toml ]; then
  crypto-election-node generate-template common.toml --validators-count $node_count
fi

for i in $(seq 0 $((node_count - 1)))
do
  if [ ! -f $((i + 1))/sec.toml ] && [ ! -f $((i + 1))/pub.toml ]; then
    peer_port=$((start_peer_port + i))
    crypto-election-node generate-config common.toml $((i + 1)) --peer-address 127.0.0.1:${peer_port} -n
  fi
done

for i in $(seq 0 $((node_count - 1)))
do
  if [ ! -f $((i + 1))/node.toml ]; then
    public_port=$((start_public_port + i))
    private_port=$((public_port + node_count))
    crypto-election-node finalize --public-api-address 0.0.0.0:${public_port} --private-api-address 0.0.0.0:${private_port} $((i + 1))/sec.toml $((i + 1))/node.toml --public-configs {1,2,3,4}/pub.toml
  fi
done

for i in $(seq 0 $((node_count - 1)))
do
  public_port=$((start_public_port + i))
  private_port=$((public_port + node_count))
  crypto-election-node run --node-config $((i + 1))/node.toml --db-path $((i + 1))/db --public-api-address 0.0.0.0:${public_port} --master-key-pass pass &
  echo "new node with ports: $public_port (public) and $private_port (private)"
  sleep 1
done

# FixMe: Launcher fails to start
echo "Deploying of crypto-election service is in progress..."
sleep 7
python3 -m exonum_launcher -i crypto-election.yml

while true; do
   sleep 300
done
