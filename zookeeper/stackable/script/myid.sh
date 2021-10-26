#!/bin/bash

if [ $# -eq 0 ]
  then
    echo "Error: no myid provided. Usage:"
    echo "    $0 <myid>"
    exit 1
fi

if ! [[ $1 =~ ^[0-9]+$ ]] ; then
   echo "Error: $1 not a number. Only integers accepted!"
   exit 1
fi


myid=/stackable/zookeeper/data/myid

echo "Writing myid=$1 to $myid..."
# we need that here? what about the hardcoded path?
touch myid
echo "$1" > myid

