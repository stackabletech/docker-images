# Support for HBase 2.6

Support for HBase 2.6 in SDP 24.7 is experimental.
As of SDP 25.3 this support is non-experimental.

## Phoenix

HBase 2.6 support [was added](https://github.com/apache/phoenix/pull/1793) with [PHOENIX-7172](https://issues.apache.org/jira/browse/PHOENIX-7172) and released with Phoenix 5.2.1, which is included since SDP 25.3.
SDP 24.7 included Phoenix built from the master branch from commit [4afe457](https://github.com/apache/phoenix/tree/4afe4579bb3ab01725e4939746d0b7b807b438ac).

## HBase operator tools

Repository: [hbase-operator-tools](https://github.com/apache/hbase-operator-tools)

This is now mirrored and built from source using `patchable`.
