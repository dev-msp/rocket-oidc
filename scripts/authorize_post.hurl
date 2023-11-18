POST http://localhost:8000/authorize

[FormParams]
response_type: code
client_id: {{client_id}}
redirect_uri: http://localhost:3000/callback
scope: openid,profile,email
state: 1234567890

HTTP 200
