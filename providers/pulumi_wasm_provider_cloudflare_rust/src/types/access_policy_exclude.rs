#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct AccessPolicyExclude {
    /// Matches any valid Access service token.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Box<Option<bool>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Box<Option<Vec<crate::types::AccessPolicyExcludeAuthContext>>>,
    /// The type of authentication method. Refer to https://datatracker.ietf.org/doc/html/rfc8176#section-2 for possible types.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "authMethod")]
    pub r#auth_method: Box<Option<String>>,
    /// Matches an Azure group. Requires an Azure identity provider.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "azures")]
    pub r#azures: Box<Option<Vec<crate::types::AccessPolicyExcludeAzure>>>,
    /// Matches any valid client certificate.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<bool>>,
    /// Matches a valid client certificate common name.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "commonName")]
    pub r#common_name: Box<Option<String>>,
    /// Overflow field if you need to have multiple common*name rules in a single policy.  Use in place of the singular common*name field.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "commonNames")]
    pub r#common_names: Box<Option<Vec<String>>>,
    /// The ID of a device posture integration.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "devicePostures")]
    pub r#device_postures: Box<Option<Vec<String>>>,
    /// The email domain to match.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "emailDomains")]
    pub r#email_domains: Box<Option<Vec<String>>>,
    /// The ID of a previously created email list.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "emailLists")]
    pub r#email_lists: Box<Option<Vec<String>>>,
    /// The email of the user.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    /// Matches everyone.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "everyone")]
    pub r#everyone: Box<Option<bool>>,
    /// Create Allow or Block policies which evaluate the user based on custom criteria. https://developers.cloudflare.com/cloudflare-one/policies/access/external-evaluation/.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "externalEvaluation")]
    pub r#external_evaluation: Box<Option<crate::types::AccessPolicyExcludeExternalEvaluation>>,
    /// Matches a specific country.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "geos")]
    pub r#geos: Box<Option<Vec<String>>>,
    /// Matches a Github organization. Requires a Github identity provider.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "githubs")]
    pub r#githubs: Box<Option<Vec<crate::types::AccessPolicyExcludeGithub>>>,
    /// The ID of a previously created Access group.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<String>>>,
    /// Matches a group in Google Workspace. Requires a Google Workspace identity provider.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "gsuites")]
    pub r#gsuites: Box<Option<Vec<crate::types::AccessPolicyExcludeGsuite>>>,
    /// The ID of a previously created IP list.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Box<Option<Vec<String>>>,
    /// An IPv4 or IPv6 CIDR block.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "ips")]
    pub r#ips: Box<Option<Vec<String>>>,
    /// The ID of a configured identity provider.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "loginMethods")]
    pub r#login_methods: Box<Option<Vec<String>>>,
    /// Matches an Okta group. Requires an Okta identity provider.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "oktas")]
    pub r#oktas: Box<Option<Vec<crate::types::AccessPolicyExcludeOkta>>>,
    /// Matches a SAML group. Requires a SAML identity provider.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "samls")]
    pub r#samls: Box<Option<Vec<crate::types::AccessPolicyExcludeSaml>>>,
    /// The ID of an Access service token.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Box<Option<Vec<String>>>,
}
