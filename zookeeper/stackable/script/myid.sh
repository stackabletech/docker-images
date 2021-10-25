#!/bin/bash

if [ $# -eq 0 ]
  then
    echo "Error: no myid provided. Usage:"
    echo "    $0 <myid>"
    exit 1
fi

re=''
if ! [[ $1 =~ ^[0-9]+$ ]] ; then
   echo "Error: $1 not a number. Only integers accepted!"
   exit 1
fi

# we need that here? what about the hardcoded path?
touch /stackable/zookeeper/data/myid
echo "$1" > /stackable/zookeeper/data/myid

