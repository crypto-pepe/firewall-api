# Unban target

Unbans all targets or target with provided details.

**URL** : `/api/unban`

**Method** : `POST`

### Request constraints

**Content-Type**: `application/json`

| Field    | Type                                             | Is required | Note                                              |
|----------|--------------------------------------------------|-------------|---------------------------------------------------|
| `target` | `{ ip: string?, user_agent:string? } &#124; '*'` | Yes         | Ban target. If target is "*" - unbans all targets |

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

**Condition** : Target was unbanned by all executors

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
    "reason": "Some executors didn't unban successfully",
    "details": {
        "executor_1": "OK",
        "executor_2": "500 INTERNAL SERVER ERROR"
    }
}
```
