#!/bin/bash
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
} >>"${ENV_FILE}"

docker-compose up --build --force-recreate --no-deps -d ci

rm .env
