#!/bin/bash

previewHash=$(jq -r '.previewHash' config.json)
quickstartHash=$(jq -r '.quickstartHash' config.json)
projectName=$(jq -r '.projectName' config.json)

previewVersion=$(echo "$previewHash" | cut -d'@' -f1)
echo $previewVersion

set -e

case "$1" in
standalone)
    echo "Using standalone network"
    ARGS="--local --enable-soroban-diagnostic-events"
    ;;
futurenet)
    echo "Using Futurenet network"
    ARGS="--futurenet"
    ;;
testnet)
    echo "Using Testnet network"
    ARGS="--testnet"
    ;;
*)
    echo "Usage: $0 standalone|futurenet|testnet"
    exit 1
    ;;
esac

shift

echo "1. Creating docker soroban network"
(docker network inspect soroban-network -f '{{.Id}}' 2>/dev/null) \
  || docker network create soroban-network

echo "  "
echo "  "

echo "2. Searching for a previous soroban-preview docker container"
containerID=$(docker ps --filter=`name=soroban-preview-${previewVersion}-${projectName}` --all --quiet)
if [[ ${containerID} ]]; then
    echo "Start removing soroban-preview-${previewVersion}-${projectName}  container."
    docker rm --force soroban-preview-${previewVersion}-${projectName}
    echo "Finished removing soroban-preview-${previewVersion}-${projectName} container."
else
    echo "No previous soroban-preview-${previewVersion}-${projectName} container was found"
fi
echo "  "
echo "  "

echo "3. Searching for a previous stellar container"
containerID=$(docker ps --filter=`name=stellar-${projectName}` --all --quiet)
if [[ ${containerID} ]]; then
    echo "Start removing stellar-${projectName} container."
    docker rm --force stellar-${projectName}
    echo "Finished removing stellar-${projectName} container."
else
    echo "No previous stellar-${projectName} container was found"
fi
echo "  "
echo "  "

echo "4. Run a soroban-preview-${previewVersion}-${projectName} container"

currentDir=$(pwd)
docker run -dti \
  --volume ${currentDir}:/workspace \
  --name soroban-preview-${previewVersion}-${projectName} \
  -p 7001:8000 \
  --ipc=host \
  --network soroban-network \
  esteblock/soroban-preview:${previewHash}

echo "  "
echo "  "

echo "5. Run a stellar-${projectName} quickstart container"
# Run the stellar-${projectName} quickstart image
docker run --rm -ti \
  --name stellar-${projectName} \
  --network soroban-network \
  -p 7000:8000 \
  stellar/quickstart:${quickstartHash} \
  $ARGS \
  --enable-soroban-rpc \
  --protocol-version 20 \
  --enable-soroban-diagnostic-events \
  "$@" # Pass through args from the CLI
