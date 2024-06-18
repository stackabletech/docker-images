# Support for HBase 2.6

## Phoenix

At the time of this writing (June 10, 2024) there is no Phoenix release that supports HBase 2.6 so the script `upload_new_phoenix_version.sh` will not work.

This [pull request](https://github.com/apache/phoenix/pull/1793) added support for HBase 2.6 and we buid from that merge commit (ce17ec1da).

```bash
# clone the Phoenix repo
git clone git@github.com:apache/phoenix.git
cd phoenix
git checkout ce17ec1da

# create a tarball
mkdir ../phoenix-5.3.0-SNAPSHOT
git archive --format=tar --output ../phoenix-5.3.0-SNAPSHOT/phoenix.tar ce17ec1da
cd ../phoenix-5.3.0-SNAPSHOT
tar xf phoenix.tar
rm phoenix.tar
echo ce17ec1da > git-commit
cd ..
tar -c phoenix-5.3.0-SNAPSHOT | gzip > phoenix-5.3.0-SNAPSHOT-src.tar.gz
```

## HBase operator tools

Repository: [hbase-operator-tools](https://github.com/apache/hbase-operator-tools)

Built from git hash [4286235](https://github.com/apache/hbase-operator-tools/commit/428623538a8b486762b83b098328510a53db54fe) (main)
since no release supporting HBase 2.6 available yet.

```bash
mkdir ../hbase-operator-tools-1.3.0-SNAPSHOT
git archive --format=tar --output ../hbase-operator-tools-1.3.0-SNAPSHOT/hot.tar 4286235
cd ../hbase-operator-tools-1.3.0-SNAPSHOT
tar xf hot.tar
echo 4286235 > git-commit
cd ..
tar c hbase-operator-tools-1.3.0-SNAPSHOT|gzip > hbase-operator-tools-1.3.0-SNAPSHOT-src.tar.gz
```
