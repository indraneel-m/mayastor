from common.hdl import MayastorHandle
from common.command import run_cmd, run_cmd_async
from common.nvme import nvme_connect, nvme_disconnect
from common.fio import Fio
from common.fio_spdk import FioSpdk
from common.mayastor import containers, mayastors, create_temp_files, check_size
import pytest
import asyncio
import uuid as guid
import time
import subprocess
import mayastor_pb2 as pb


NEXUS_COUNT = 10
NEXUS_SIZE = 500 * 1024 * 1024
REPL_SIZE = NEXUS_SIZE
POOL_SIZE = REPL_SIZE * NEXUS_COUNT + 100 * 1024 * 1024

@pytest.fixture
def local_files(mayastors):
    files = []
    for name, ms in mayastors.items():
        path = f"/tmp/disk-{name}.img"
        pool_size_mb = int(POOL_SIZE / 1024 / 1024)
        subprocess.run(
            ["sudo", "sh", "-c", f"rm -f '{path}'; truncate -s {pool_size_mb}M '{path}'"],
            check=True,
        )
        files.append(path)

    yield
    for path in files:
        subprocess.run(["sudo", "rm", "-f", path], check=True)


@pytest.fixture
def create_replicas_on_all_nodes(local_files, mayastors, create_temp_files):
    uuids = []

    for name, ms in mayastors.items():
        ms.pool_create(name, f"aio:///tmp/disk-{name}.img")
        # verify we have zero replicas
        assert len(ms.replica_list().replicas) == 0

    for i in range(NEXUS_COUNT):
        uuid = guid.uuid4()
        for name, ms in mayastors.items():
            before = ms.pool_list()
            ms.replica_create(name, uuid, REPL_SIZE)
            after = ms.pool_list()
        uuids.append(uuid)

    yield uuids


@pytest.fixture
def create_nexuses(mayastors, create_replicas_on_all_nodes):
    nexuses = []
    nexuses_uris = []

    uris = [
        [replica.uri for replica in mayastors.get(node).replica_list().replicas]
        for node in ["ms1", "ms2", "ms3"]
    ]
    
    node_index = 0
    for children in zip(*uris):
        ms = mayastors.get(f"ms{node_index}")
        uuid = guid.uuid4()
        nexus = ms.nexus_create(uuid, NEXUS_SIZE, list(children))
        nexuses.append(nexus)
        nexuses_uris.append(ms.nexus_publish(uuid))
        # uncomment to spread nexuses
        #node_index += 1
        if node_index == 4:
            node_index = 0

    yield nexuses

    # for nexus in ms1.nexus_list():
    #     uuid = nexus.uuid
    #     ms1.nexus_unpublish(uuid)
    #     ms1.nexus_destroy(uuid)
    # for nexus in ms0.nexus_list():
    #     uuid = nexus.uuid
    #     ms0.nexus_unpublish(uuid)
    #     ms0.nexus_destroy(uuid)


@pytest.mark.parametrize("times", range(1))
def test_rebuild_failure(containers, mayastors, times, create_nexuses):
    for node_index in range(0, 4):
        ms = mayastors.get(f"ms{node_index}")
        for nexus in ms.nexus_list():
            print(nexus)

    
    ms1 = mayastors.get("ms1")
    ms2 = mayastors.get("ms2")
    ms3 = mayastors.get("ms3")

    node = containers.get("ms3")
    node.stop()
    time.sleep(5)
    node.start()

    # must reconnect grpc
    ms3.reconnect()
    # must recreate the pool for import
    ms3.pool_create("ms1", "aio:///tmp/disk-ms1.img")
    time.sleep(1)

    # check the list has the required number of replicas
    replicas = ms1.replica_list().replicas
    assert len(replicas) == NEXUS_COUNT

    for node_index in range(0, 4):
        ms = mayastors.get(f"ms{node_index}")
        # add the replicas to the nexuses for rebuild
        for nexus in ms.nexus_list():
            child = list(filter(lambda child: child.state == pb.CHILD_FAULTED, list(nexus.children)))[0]
            if nexus.state != pb.NEXUS_FAULTED:
                try:
                    ms.nexus_remove_replica(nexus.uuid, child.uri)
                    ms.nexus_add_replica(nexus.uuid, child.uri)
                except:
                    print(f"Failed to remove child {child.uri} from {nexus}")

    ms0 = mayastors.get("ms0")
    for i in range(5):
        ms0.nexus_list()    
        time.sleep(1)

    node.stop()

    for i in range(40):
        ms0.nexus_list()
        time.sleep(1)
