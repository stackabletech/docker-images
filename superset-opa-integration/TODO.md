# ToDos

- Test with UIF + caching in OPA (+ document this)
- Implement changes in operator and CRD
- Documentation (how to write rego rules for this)
- Tests
- "Sync interval" mechanism in superset to improve latency and not spam OPA
- Mount OPA service discovery configMap

# What is working
- User-role sync OPA -> Superset
- Role-sync OPA -> Superset during user-role sync
- OPA rego rules returning user-to-role mappings
- Error handling in case OPA is not available or the API call returns a non 200 code or if the result data is garbage (e.g. the UIF source system is not available)
- Make OPA address etc configurable via service discovery
- Create a patch for superset image with OPA integration
