import superalign


def test_schema_version():
    version = superalign.core.schema_version()
    assert isinstance(version, str)
    assert version == "1.0.0"


def test_provenance_manager():
    pm = superalign.ProvenanceManager(pipeline_version="0.1.0")
    parse_id = pm.record_process(
        operation="TEST_OP", inputs=["hash1"], outputs=["hash2"]
    )
    assert isinstance(parse_id, str)
    report = pm.to_json_report()
    assert "TEST_OP" in report
