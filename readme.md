# Firewall-api

Implements `POST /api/check-ban`
from [this](https://github.com/crypto-pepe/firewall/wiki/Banned-Targets#check-targets-ban)
api

## ENVs

| Name        | Required | Note                                                                                  |
|-------------|----------|---------------------------------------------------------------------------------------|
| RUST_LOG    | No       | Log level. https://docs.rs/env_logger/0.9.0/env_logger/#enabling-logging              |
| CONFIG_PATH | No       | Path to the `yaml` formatted config file. If not stated, `./config.yaml` will be used |

## Config

| Name                      | Required | Note                          |
|---------------------------|----------|-------------------------------|
| redis.host                | Yes      | Redis service host            |
| redis.port                | Yes      | Redis service port            |
| redis.timeout_sec         | Yes      | Redis query timeout (seconds) |
| redis.client_id           | No       | Redis client id               |
| redis.password            | No       | Redis password                |
| server.host               | Yes      | Firewall-api service host     |
| server.port               | Yes      | Firewall-api service port     |
| telemetry.svc_name        | Yes      | Service name for tracing      |
| telemetry.jaeger_endpoint | No       | Jaeger endpoint               |

