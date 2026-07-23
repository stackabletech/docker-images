def test_openlineage_symbols_exist() -> None:
    from openlineage.client import OpenLineageClient  # noqa: F401
    from openlineage.client.event_v2 import (  # noqa: F401
        InputDataset,
        Job,
        OutputDataset,
        Run,
        RunEvent,
        RunState,
    )
    from openlineage.client.facet_v2 import (  # noqa: F401
        column_lineage_dataset,
        job_type_job,
        schema_dataset,
        sql_job,
    )
    from openlineage.client.uuid import generate_new_uuid

    assert str(generate_new_uuid())
    assert RunState.START
    assert RunState.COMPLETE


def test_package_imports() -> None:
    import superset_openlineage  # noqa: F401
