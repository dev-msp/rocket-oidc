use entity::clients::ResponseType;

#[derive(Debug, FromFormField)]
pub enum Prompt {
    /// The Authorization Server MUST NOT display any authentication or consent user interface
    /// pages. An error is returned if the End-User is not already authenticated or the Client does
    /// not have pre-configured consent for the requested Claims or does not fulfill other
    /// conditions for processing the request. The error code will typically be `login_required`,
    /// `interaction_required`, or another code defined in Section 3.1.2.6. This can be used as a
    /// method to check for existing authentication and/or consent.
    None,

    /// The Authorization Server SHOULD prompt the End-User for reauthentication. If it cannot
    /// reauthenticate the End-User, it MUST return an error, typically `login_required`.
    Login,

    /// The Authorization Server SHOULD prompt the End-User for consent before returning
    /// information to the Client. If it cannot obtain consent, it MUST return an error, typically
    /// `consent_required`.
    Consent,

    /// The Authorization Server SHOULD prompt the End-User to select a user account. This enables
    /// an End-User who has multiple accounts at the Authorization Server to select amongst the
    /// multiple accounts that they might have current sessions for. If it cannot obtain an account
    /// selection choice made by the End-User, it MUST return an error, typically
    /// `account_selection_required`.
    SelectAccount,
}

#[derive(FromForm)]
pub struct AuthorizePayload {
    /// OAuth 2.0 Response Type value that determines the authorization processing flow to be used,
    /// including what parameters are returned from the endpoints used. When using the
    /// authorization code flow, this value is `code`.
    response_type: ResponseType,

    /// OAuth 2.0 Client Identifier valid at the Authorization Server.
    client_id: String,

    /// Redirection URI to which the response will be sent. This URI MUST exactly match one of the
    /// Redirection URI values for the Client pre-registered at the OpenID Provider, with the
    /// matching performed as described in Section 6.2.1 of [RFC3986] (Simple String Comparison).
    /// When using this flow, the Redirection URI SHOULD use the https scheme; however, it MAY use
    /// the http scheme, provided that the Client Type is confidential, as defined in Section 2.1
    /// of OAuth 2.0, and provided the OP allows the use of http Redirection URIs in this case. The
    /// Redirection URI MAY use an alternate scheme, such as one that is intended to identify a
    /// callback into a native application.
    redirect_uri: String,

    /// OpenID Connect requests MUST contain the `openid` scope value. If the `openid` scope value
    /// is not present, the behavior is entirely unspecified. Other scope values MAY be present.
    /// Scope values used that are not understood by an implementation SHOULD be ignored. See
    /// Sections 5.4 and 11 for additional scope values defined by this specification.
    scope: String,

    /// Opaque value used to maintain state between the request and the callback. Typically,
    /// Cross-Site Request Forgery (CSRF, XSRF) mitigation is done by cryptographically binding the
    /// value of this parameter with a browser cookie.
    state: String,

    /// Space delimited, case sensitive list of ASCII string values that specifies whether the
    /// Authorization Server prompts the End-User for reauthentication and consent.
    prompt: Option<Prompt>,

    /// String value used to associate a Client session with an ID Token, and to mitigate replay
    /// attacks. The value is passed through unmodified from the Authentication Request to the ID
    /// Token. Sufficient entropy MUST be present in the nonce values used to prevent attackers
    /// from guessing values. For implementation notes, see Section 15.5.2.
    nonce: Option<String>,
}

impl AuthorizePayload {
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn response_type(&self) -> &ResponseType {
        &self.response_type
    }
}

impl ToString for AuthorizePayload {
    fn to_string(&self) -> String {
        format!(
            "response_type={:?}, client_id={}, redirect_uri={}, scope={}, state={}, prompt={:?}, nonce={:?}",
            self.response_type,
            self.client_id,
            self.redirect_uri,
            self.scope,
            self.state,
			self.prompt,
            self.nonce
        )
    }
}
