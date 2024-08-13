# Stackable Docker image including Apache Hive

This is our Dockerfile for the Apache Hive metastore.

It is building Hive from source.
We use the officially released source tarballs and apply patches on top.
These patches can also be seen in our [fork](https://github.com/stackabletech/hive) of Hive.

Look for the `stackable/` branches.

The patches were created using [Stacked Git](https://stacked-git.github.io/) but that, unfortunately, does not allow sharing the state remotely.
We do not have a good solution for this yet.

The command used was: `stg export --dir patches/3.1.3 -p -n` but this would require you to get your local fork into a state where it recognizes all _our_ commits as patches first.
