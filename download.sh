#!/bin/bash

if [ "$(uname)" == 'Darwin' ]; then
  OS='apple'
elif [ "$(expr substr $(uname -s) 1 5)" == 'Linux' ]; then
  OS='linux'
elif [ "$(expr substr $(uname -s) 1 10)" == 'MINGW32_NT' ]; then
  OS='windows'
else
  echo "Your platform ($(uname -a)) is not supported."
  exit 1
fi

curl -s https://api.github.com/repos/Lucky3028/del-ghcr/releases \
  | grep "browser_download_url" \
  | cut -d : -f 2,3 \
  | tr -d \" \
  | grep $OS \
  | xargs wget -O del-ghcr.gz

gzip -d del-ghcr.gz
