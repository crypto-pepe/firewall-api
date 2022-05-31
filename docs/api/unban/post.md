# Unban target

Unbans all targets or target with provided details for each executor

**URL** : `/api/unban`

**Method** : `POST`

### Request constraints

**Content-Type**: `application/json`

| Field    | Type                                        | Is required | Note                                              |
|----------|---------------------------------------------|------------|---------------------------------------------------|
| `target` | `{ ip: string?, user_agent:string? } \| '*'`       | Yes         | Ban target. If target is "*" - unbans all targets |

**Request examples**

```json
{
    "target": {
        "ip": "11.12.13.14",
        "user_agent": "curl user-agent"
    }
}
```

```json
{
    "target": "*"
}
```

## Success Response

**Condition** : Ban was successfully applied.

**Code** : `204 NO CONTENT`

## Error Responses

**Condition** : If fields are missed.

**Code** : `400 BAD REQUEST`

**Body example**

```json
{
    "code": 400,
    "reason": "Provided request does not match the constraints",
    "details": {
        "target": "This field is required"
    }
}
```

**Condition** : If one or multiple executors returned error

**Code** : `500 INTERNAL SERVER ERROR`

**Body example**

```json
{
    "code": 500,
    "reason": "Some executors didn't response with success",
    "details": {
        "executor_2": "Internal server error"
    }
}
```
