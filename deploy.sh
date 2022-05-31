#!/bin/bash
# DOCKER_BUILDKIT=1 BUILDKIT_PROGRESS=plain ./deploy.sh -t dev -a "/home/roman/.ssh/mini-ci" -p "a"
set -o errexit -o pipefail -o noclobber -o nounset

while getopts ":t:p:a:" flag; do
  case "${flag}" in
  t) target=${OPTARG} ;;
  p) ssh_password=${OPTARG} ;;
  a) ssh_path=${OPTARG} ;;
  *)
    echo "Unknown parameter passed: $1"
    exit 1
    ;;
  esac
done

echo "Deploying target: $target"

ENV_FILE=.env

rm -f "${ENV_FILE}"
touch "${ENV_FILE}"

{
  echo "ENV_PROFILE=$target"
  echo "SSH_KEY_PATH=$ssh_path"
  echo "SSH_KEY_PASSWORD=$ssh_password"
  echo "DOCKER_GROUP_ID=$(getent group docker | cut -d: -f3)"
} >>"${ENV_FILE}"

DOCKER_BUILDKIT=1 BUILDKIT_PROGRESS=plain docker-compose up \
  --build --force-recreate --no-deps -d ci

rm .env
