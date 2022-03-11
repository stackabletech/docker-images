#!/usr/bin/env python3
import nipyapi
import argparse
import sys

# no stack trace
sys.tracebacklimit = 0


def init(url: str, username: str, password: str, ca_file: str):
    nipyapi.config.nifi_config.host = url
    nipyapi.security.set_service_ssl_context(service='nifi', ca_file=ca_file)

    try:
        nipyapi.security.service_login(service='nifi', username=username, password=password)
        print("Successfully authenticated and established connection with [%s]" % url)
    except Exception as e:
        raise Exception("Failed to connect to {}: {}".format(url, str(e))) from None
        exit(-1)


def find_reporting_task(name: str, port: str):
    flow_api = nipyapi.nifi.apis.flow_api.FlowApi()

    try:
        reporting_tasks = flow_api.get_reporting_tasks().reporting_tasks
    except Exception as e:
        raise Exception("Could not find ReportingTask[{}/{}]: {}".format(name, port, str(e))) from None
        exit(-1)

    for task in reporting_tasks:
        task_dict = task.to_dict()
        task_name = task_dict["component"]["name"]
        task_port = task_dict["component"]["properties"]["prometheus-reporting-task-metrics-endpoint-port"]
        if task_name == name and task_port == port:
            return task

    return None


def create_reporting_task(name: str, port: str, version: str):
    task = nipyapi.nifi.models.reporting_task_entity.ReportingTaskEntity(
        revision=nipyapi.nifi.models.revision_dto.RevisionDTO(version=0),
        disconnected_node_acknowledged=False,
        component=nipyapi.nifi.models.reporting_task_dto.ReportingTaskDTO(
            name=name,
            type="org.apache.nifi.reporting.prometheus.PrometheusReportingTask",
            bundle=nipyapi.nifi.models.bundle_dto.BundleDTO(
                group="org.apache.nifi",
                artifact="nifi-prometheus-nar",
                version=version
            ),
            properties={
                "prometheus-reporting-task-metrics-endpoint-port": port,
                "prometheus-reporting-task-metrics-send-jvm": True
            }
        )
    )

    controller_api = nipyapi.nifi.apis.controller_api.ControllerApi()

    try:
        return controller_api.create_reporting_task(body=task)
    except Exception as e:
        raise Exception("Could not create reporting task: {}".format(str(e))) from None
        exit(-1)


def get_reporting_task_id(task):
    return task.id


def get_reporting_task_name(task):
    task_dict = task.to_dict()
    return task_dict["component"]["name"]


def get_revision_version(task):
    task_dict = task.to_dict()
    return task_dict["revision"]["version"]


def is_reporting_task_running(task):
    task_dict = task.to_dict()
    return task_dict["component"]["state"] == "RUNNING"


def set_reporting_task_running(task):
    reporting_task_api = nipyapi.nifi.apis.reporting_tasks_api.ReportingTasksApi()

    state = {
        "revision": {
            "version": get_revision_version(task)
        },
        "disconnected_node_acknowledged": False,
        "state": "RUNNING"
    }

    try:
        return reporting_task_api.update_run_status(id=get_reporting_task_id(task), body=state)
    except Exception as e:
        raise Exception("Could not set ReportingTask [{}] to RUNNING: {}".format(get_reporting_task_id(task), str(e))) from None
        exit(-1)


def main():
    # Construct an argument parser
    all_args = argparse.ArgumentParser()
    # Add arguments to the parser
    all_args.add_argument("-n", "--nifi_api_url", required=True,
                          help="The NiFi node url to connect to.")
    all_args.add_argument("-u", "--username", required=True,
                          help="Username to connect as.")
    all_args.add_argument("-p", "--password", required=True,
                          help="Password for the user.")
    all_args.add_argument("-v", "--nifi_version", required=True,
                          help="The NiFi product version.")
    all_args.add_argument("-c", "--cert", required=False, default="/tmp/ca_cert.pem",
                          help="The path to the public certificate.")
    all_args.add_argument("-m", "--metrics_port", required=False, default="9505",
                          help="Metrics port to be set in the ReportingTask.")
    all_args.add_argument("-t", "--task_name", required=False, default="StackablePrometheusReportingTask",
                          help="The name of ReportingTask to create or activate.")
    args = vars(all_args.parse_args())

    task_name = args["task_name"]
    port = args["metrics_port"]

    init(args["nifi_api_url"], args["username"], args["password"], args["cert"])

    reporting_task = find_reporting_task(name=task_name, port=port)

    if reporting_task is None:
        reporting_task = create_reporting_task(name=task_name, port=port, version=args["nifi_version"])
        print(get_reporting_task_name(task=reporting_task) + " [%s] -> CREATED" % reporting_task.id)

    if not is_reporting_task_running(task=reporting_task):
        reporting_task = set_reporting_task_running(task=reporting_task)

    print(get_reporting_task_name(task=reporting_task) + " [%s] -> RUNNING" % reporting_task.id)


if __name__ == '__main__':
    main()
