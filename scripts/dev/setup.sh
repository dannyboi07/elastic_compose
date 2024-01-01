#!/bin/bash

if ! [[ -d /var/lib/elastic_compose ]] ; then
    mkdir /var/lib/elastic_compose
fi

# Restart=always
# RestartSec=10