# Firewall-api

Implements `POST /api/check-ban`
from [this](https://github.com/crypto-pepe/firewall/wiki/Banned-Targets#check-targets-ban)
api

## ENVs

| Name        | Required | Note                                                                     |
|-------------|----------|--------------------------------------------------------------------------|
| RUST_LOG    | No       | Log level. https://docs.rs/env_logger/0.9.0/env_logger/#enabling-logging |
| CONFIG_PATH | No       | Path to the `yaml` formatted config file.                                |

## Config

**If `CONFIG_PATH` is not stated then `./config.yaml` will be used**

| Name                      | Required | Note                                                                                     |
|---------------------------|----------|------------------------------------------------------------------------------------------|
| redis.host                | Yes      | Redis service host                                                                       |
| redis.port                | Yes      | Redis service port                                                                       |
| redis.timeout_sec         | Yes      | Redis query timeout (seconds)                                                            |
| redis.client_id           | No       | Redis client id                                                                          |
| redis.password            | No       | Redis password                                                                           |
| redis.namespace           | True     | Prefix, that will be added to all keys to receive (must be same as in firewall-executor) |
| server.host               | Yes      | Firewall-api service host                                                                |
| server.port               | Yes      | Firewall-api service port                                                                |
| telemetry.svc_name        | Yes      | Service name for tracing                                                                 |
| telemetry.jaeger_endpoint | No       | Jaeger endpoint                                                                          |

___

Each of the configuration parameter can be overridden via the environment variable. Nested values overriding are
supported via the '.' separator.

Example:

| Parameter name | Env. variable |
|----------------|---------------|
| some_field     | SOME_FIELD    |
| server.port    | SERVER.PORT   |