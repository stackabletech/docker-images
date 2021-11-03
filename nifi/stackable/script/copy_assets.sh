#!/bin/bash

if ! [ $# -eq 2 ]
  then
    echo "Wrong number of arguments. Usage:"
    echo "    $0 <confmapDir> <version>"
    echo "    $0 ./conf 1.13.2"
    exit 1
fi

dir=$1
version=$2

cp $dir/* /stackable/nifi-${version}/conf
