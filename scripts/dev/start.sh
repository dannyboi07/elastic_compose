#!/bin/bash

if ! [[ -d /var/lib/elastic_compose ]] ; then
    mkdir /var/lib/elastic_compose
fi

# cargo build --release
# mv /target/release/elastic_compose /var/lib/elastic_compose/elastic_compose
# mv /target/release/elastic_compose_service /var/lib/elastic_compose/elastic_compose_service

# chmod +x /var/lib/elastic_compose/elastic_compose
# chmod +x /var/lib/elastic_compose/elastic_compose_service

# command=$1
# port=$2

# echo $1;
# echo $2;
# echo $3;
# echo $4;
# echo $5;

# for arg in "$@"; do
#     echo $arg
# done

for (( i=1; i<=$#; i++ )); do
    if [[ ${i} == 1 ]]; then
        command=${!i}
    elif [[ ${i} == 3 ]]; then
        port=${!i}
    elif [[ ${i} == 5 ]]; then
        # service=${!i}
        projects=${!i:$#}
    fi
    # elif [[ ${i} == 3 ]]; then
    #     service_port=${!i}
    # elif [[ ${i} == 4 ]]; then
    #     service_name=${!i}
    # fi
    echo ${i} ${!i}
done

echo "command: ${command}"
echo "port: ${port}"
echo "projects: ${projects}"
