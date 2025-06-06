# Support for HBase 2.6

As of SDP 24.7 we do include HBase 2.6 support in an experimental state.

## Phoenix

HBase 2.6 support [was added](https://github.com/apache/phoenix/pull/1793) with [PHOENIX-7172](https://issues.apache.org/jira/browse/PHOENIX-7172) and released with Phoenix 5.2.1, which is included since SDP 25.3.
SDP 24.7 included Phoenix built from the master branch from commit [4afe457](https://github.com/apache/phoenix/tree/4afe4579bb3ab01725e4939746d0b7b807b438ac).

## HBase operator tools

Repository: [hbase-operator-tools](https://github.com/apache/hbase-operator-tools)

Built from git hash [7c738fc](https://github.com/apache/hbase-operator-tools/tree/7c738fc1bd14fd3e2ca4e66569b496b3fd9d0288) (master)
since no release supporting HBase 2.6 available yet.

```bash
mkdir ../hbase-operator-tools-1.3.0-7c738fc
git archive --format=tar --output ../hbase-operator-tools-1.3.0-7c738fc/hot.tar 7c738fc
cd ../hbase-operator-tools-1.3.0-7c738fc
tar xf hot.tar
rm hot.tar
echo 7c738fc > git-commit
cd ..
tar -c hbase-operator-tools-1.3.0-7c738fc|gzip > hbase-operator-tools-1.3.0-7c738fc-src.tar.gz
```
