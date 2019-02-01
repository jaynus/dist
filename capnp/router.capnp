@0xb312981b2552aaff;
# Test
using Dist = import "dist.capnp";

interface UniqueIdentifierService {
    getNewId @0 () -> (id: Dist.Id);
}


enum ServiceErrorCode {
    failed @0;
    ok @1;
}
struct ServiceResult {
    code @0 : ServiceErrorCode;
    details @1 : Text;
}

enum ServiceStatusCode {
    failed @0;
    ok @1;
}

struct ServiceStatus {
    code @0 : ServiceStatusCode;
    details @1 : Text;
}

interface RouterService {
    registerWorker @0 (worker: WorkerService) -> (result: ServiceResult);
}

interface WorkerService {
    status @0 () -> (status: ServiceStatus);

    shutdown @1 () -> (result: ServiceResult);

    setRegion @2 (region: Dist.Box) -> (result: ServiceResult);
    getRegion @3 (region: Dist.Box) -> (result: ServiceResult);

    giveEntities @4 (entity: List(Dist.Id)) -> (result: ServiceResult);
    takeEntities @5 (entity: List(Dist.Id)) -> (result: ServiceResult);
}