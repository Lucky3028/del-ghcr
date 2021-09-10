#!/bin/bash
# https://qiita.com/b4b4r07/items/dcd6be0bb9c9185475bb#getopt

OPT=`getopt -o t:c:d -l token:,container:,dry-run -- "$@"`
if [ $? != 0 ]; then
  exit 1
fi
eval set -- "$OPT"

while true
do
  case $1 in
    -t | --token)
      readonly GHCR_TOKEN=$2
      shift 2
      ;;
    -c | --container)
      readonly CONTAINER_NAME=$2
      shift 2
      ;;
    -d | --dry-run)
      readonly IS_DRY_RUN=true
      shift
      ;;
    --)
      shift
      break
      ;;
    *)
      echo "Internal error!" 1>&2
      exit 1
      ;;
  esac
done

del-ghcr -t ${GHCR_TOKEN:?} -c ${CONTAINER_NAME:?} $(if "${IS_DRY_RUN=false}"; then echo "-d"; else echo ""; fi)