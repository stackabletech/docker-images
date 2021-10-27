#!/bin/bash

if ! [ $# -eq 2 ]
  then
    echo "Wrong number of arguments. Usage:"
    echo "    $0 <dataDir> <myid>"
    echo "    $0 /tmp/zookeeper 1"
    exit 1
fi

directory=$1
id=$2

if ! [[ $directory =~ ^/|(/[\w-]+)+$ ]] ; then
   echo "Error: $directory not a directory!"
   exit 1
fi

if ! [[ $id =~ ^[0-9]+$ ]] ; then
   echo "Error: $id not a number. Only integers accepted!"
   exit 1
fi

if ! [[ -d $directory ]]; then
  echo "Directory $directory does not exist. Creating it!"
  mkdir -p "$directory"
fi

myid=$directory/myid

echo "Writing myid [$id] to $myid ..."
echo "$id" > myid

