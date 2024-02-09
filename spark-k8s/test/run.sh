
docker run --rm -ti -v "$PWD":/test docker.stackable.tech/sandbox/spark-k8s:3.5.0-stackable0.0.0-delta3.1.0 bash -c 'pip install delta-spark==3.1.0; pip install importlib-metadata; /stackable/spark/bin/spark-submit  --conf "spark.sql.warehouse.dir=/tmp" /test/test.py'


