POST http://localhost:8000/clients

{
  "name": "My Client",
  "description": "My Client Description",
  "redirect_uris": ["http://localhost:3000/callback"],
  "grant_types": ["authorization_code", "refresh_token"],
  "response_types": ["code"],
  "scope": ["openid", "profile", "email"]
}
