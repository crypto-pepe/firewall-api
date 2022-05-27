# Enable/disable dry run mode for executors

Enables or disables dry run mode for each executor

**URL** : `/api/enable-dry-run`

**Method** : `POST`

### Request constraints

**Content-Type**: `application/json`

| Field     | Type   | Required |
|-----------|--------|----------|
| `enabled` | `bool` | Yes      |

**Request examples**

```json
{
    "enabled": true
}
```

## Success Response

**Condition** : Dry run was enabled/disabled on each executor

**Code** : `204 NO CONTENT`

## Error Responses

**Condition** : Request doesn't match the constraints

**Code** : `400 BAD REQUEST`

**Body example**

```json
{
    "code": 400,
    "reason": "Provided request does not match the constraints",
    "details": {
        "enabled": "This field is required"
    }
}
```

**Condition** : If one or multiple executors returned error

**Code** : `500 INTERNAL SERVER ERROR`

**Body example**

```json
{
    "code": 500,
    "reason": "Some executors didn't set dry run successfully",
    "details": {
        "executor_1": "OK",
        "executor_2": "500 INTERNAL SERVER ERROR"
    }
}
```
