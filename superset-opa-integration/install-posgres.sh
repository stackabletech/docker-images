helm install --wait superset bitnami/postgresql \
    --set auth.username=superset \
    --set auth.password=superset \
    --set auth.database=superset
