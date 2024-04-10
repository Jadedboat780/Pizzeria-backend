POST  http://localhost:3000/authorize
Content-Type: application/json
{
    "client_id":"foo1",
    "client_secret":"super_secret_key"
}

HTTP 200
[Captures]
token: jsonpath "$.access_token"

POST http://localhost:3000/users
Authorization: Bearer {{token}}
Content-Type: application/json
{
    "username": "nikitadsa",
    "email": "foo@madil.com",
    "password": "12345678ds"
}

HTTP 200