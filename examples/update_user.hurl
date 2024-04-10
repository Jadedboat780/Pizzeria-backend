POST  http://localhost:3000/authorize
Content-Type: application/json
{
    "client_id":"foo",
    "client_secret":"super_secret_key"
}

HTTP 200
[Captures]
token: jsonpath "$.access_token"

PATCH http://localhost:3000/users/1
Authorization: Bearer {{token}}
Content-Type: application/json
{
    "username": "nikita12",
    "password": "new_p",
    "address": "hello"
}

HTTP 200