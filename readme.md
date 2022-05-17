# Firewall-api

Implements `POST /api/check-ban`
from [this](https://github.com/crypto-pepe/firewall/wiki/Banned-Targets#check-targets-ban)
api

## ENVs

| Name        | Required | Note                                                                     |
|-------------|----------|--------------------------------------------------------------------------|
| RUST_LOG    | No       | Log level. https://docs.rs/env_logger/0.9.0/env_logger/#enabling-logging |
| CONFIG_PATH | No       | Path to the `yaml` formatted config file                                 |

## Config

```yaml
redis:
  host: '127.0.0.1'
  port: 6379
  timeout_sec: 2
server:
  host: '127.0.0.1'
  port: 8000
telemetry:
  svc_name: "firewall-api"
  jaeger_endpoint: "localhost:6831"
```

| Name                      | Required | Note                               |
|---------------------------|----------|------------------------------------|
| redis.host                | Yes      | Redis service host                 |
| redis.port                | Yes      | Redis service port                 |
| redis.timeout_sec         | Yes      | Redis connection timeout (seconds) |
| server.host               | Yes      | Firewall-api service host          |
| server.port               | Yes      | Firewall-api service port          |
| telemetry.svc_name        | Yes      | Service name for tracing           |
| telemetry.jaeger_endpoint | No       | Jaeger endpoint                    |

