# Support for HBase 2.6

As of SDP release 25.3, HBase 2.6.x is fully supported.

## Phoenix

HBase 2.6 support [was added](https://github.com/apache/phoenix/pull/1793) with [PHOENIX-7172](https://issues.apache.org/jira/browse/PHOENIX-7172) and released with Phoenix 5.2.1, which is included since SDP 25.3.
SDP 24.7 included Phoenix built from the master branch from commit [4afe457](https://github.com/apache/phoenix/tree/4afe4579bb3ab01725e4939746d0b7b807b438ac).

## HBase operator tools

Repository: [hbase-operator-tools](https://github.com/apache/hbase-operator-tools)

This is now mirrored and built from source using `patchable`.

## Back-porting upstream changes

Care needs to be taken when back-porting upstream changes due to changing dependencies.
A patch for multiple versions (that share the same major.minor version) may not work if dependencies have been introduced in the meantime.
e.g. HBASE-29797 can be applied to 2.6.4 directly, but not to 2.6.3 as test libraries were introduced between those versions.
