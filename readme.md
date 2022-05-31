# Firewall-api

## ENVs

| Name        | Required | Note                                                                     |
|-------------|----------|--------------------------------------------------------------------------|
| RUST_LOG    | No       | Log level. https://docs.rs/env_logger/0.9.0/env_logger/#enabling-logging |
| CONFIG_PATH | No       | Path to the `yaml` formatted config file.                                |

## Config

**If `CONFIG_PATH` is not stated then `./config.yaml` will be used**

| Name                      | Type                               | Required | Default | Note                                                                                     |
|---------------------------|------------------------------------|----------|---------|------------------------------------------------------------------------------------------|
| redis.host                | string                             | Yes      |         | Redis service host                                                                       |
| redis.port                | int                                | Yes      |         | Redis service port                                                                       |
| redis.client_id           | string                             | No       |         | Redis client id                                                                          |
| redis.password            | string                             | No       |         | Redis password                                                                           |
| redis_query_timeout       | string                             | No       | 5s      | Redis query timeout. Duration string                                                     |
| redis_keys_prefix         | string                             | Yes      |         | Prefix, that will be added to all keys to receive (must be same as in firewall-executor) |
| server.host               | string                             | Yes      |         | Firewall-api service host                                                                |
| server.port               | int                                | Yes      |         | Firewall-api service port                                                                |
| telemetry.svc_name        | string                             | Yes      |         | Service name for tracing                                                                 |
| telemetry.jaeger_endpoint | string                             | No       |         | Jaeger endpoint                                                                          |
| executors                 | []{name: string, base_url: string} | Yes      |         | List of executors                                                                        |

___

Each of the configuration parameter can be overridden via the environment variable. Nested values overriding are
supported via the '.' separator.

Example:

| Parameter name | Env. variable |
|----------------|---------------|
| some_field     | SOME_FIELD    |
| server.port    | SERVER.PORT   |
