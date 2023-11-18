POST http://localhost:8000/authorize

[FormParams]
response_type: code
client_id: 1234567890
redirect_uri: http://localhost:8000/callback
scope: openid profile email
state: 1234567890

HTTP 200
