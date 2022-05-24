# Check target's ban

Checks whether target is banned.

**URL** : `/api/check-ban`

**Method** : `POST`

### Request constraints

**Content-Type**: `application/json`

| Field    | Type                                  | Required | Note       |
|----------|---------------------------------------|----------|------------|
| `target` | `{ ip: string?, user_agent:string? }` | Yes      | Ban target |

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
    "target": {
        "ip": "1.1.1.1"
    }
}
```

## Success Response

**Condition** : Target ban checked

**Code** : `200 OK`

**Body**

```typescript
type Response = {
    status: "banned";
    ban_expires_at: number;
} | {
    status: "free";
};
```

**Body example**

```json
{
    "status": "free"
}
```

```json
{
    "status": "banned",
    "ban_expires_at": 1651846618
}
```

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
