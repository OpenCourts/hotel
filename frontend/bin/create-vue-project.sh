#!/usr/bin/env bash

DIR_SCRIPT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

docker-compose -f "${DIR_SCRIPT}/../docker-compose.yml" run --rm -u $UID node vue create -n .