#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccessPolicyExclude {
    /// Matches any valid Access service token.
    #[builder(into, default)]
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Box<Option<Vec<super::types::AccessPolicyExcludeAuthContext>>>,
    /// The type of authentication method. Refer to https://datatracker.ietf.org/doc/html/rfc8176#section-2 for possible types.
    #[builder(into, default)]
    #[serde(rename = "authMethod")]
    pub r#auth_method: Box<Option<String>>,
    /// Matches an Azure group. Requires an Azure identity provider.
    #[builder(into, default)]
    #[serde(rename = "azures")]
    pub r#azures: Box<Option<Vec<super::types::AccessPolicyExcludeAzure>>>,
    /// Matches any valid client certificate.
    #[builder(into, default)]
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<bool>>,
    /// Matches a valid client certificate common name.
    #[builder(into, default)]
    #[serde(rename = "commonName")]
    pub r#common_name: Box<Option<String>>,
    /// Overflow field if you need to have multiple common*name rules in a single policy.  Use in place of the singular common*name field.
    #[builder(into, default)]
    #[serde(rename = "commonNames")]
    pub r#common_names: Box<Option<Vec<String>>>,
    /// The ID of a device posture integration.
    #[builder(into, default)]
    #[serde(rename = "devicePostures")]
    pub r#device_postures: Box<Option<Vec<String>>>,
    /// The email domain to match.
    #[builder(into, default)]
    #[serde(rename = "emailDomains")]
    pub r#email_domains: Box<Option<Vec<String>>>,
    /// The ID of a previously created email list.
    #[builder(into, default)]
    #[serde(rename = "emailLists")]
    pub r#email_lists: Box<Option<Vec<String>>>,
    /// The email of the user.
    #[builder(into, default)]
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    /// Matches everyone.
    #[builder(into, default)]
    #[serde(rename = "everyone")]
    pub r#everyone: Box<Option<bool>>,
    /// Create Allow or Block policies which evaluate the user based on custom criteria. https://developers.cloudflare.com/cloudflare-one/policies/access/external-evaluation/.
    #[builder(into, default)]
    #[serde(rename = "externalEvaluation")]
    pub r#external_evaluation: Box<Option<super::types::AccessPolicyExcludeExternalEvaluation>>,
    /// Matches a specific country.
    #[builder(into, default)]
    #[serde(rename = "geos")]
    pub r#geos: Box<Option<Vec<String>>>,
    /// Matches a Github organization. Requires a Github identity provider.
    #[builder(into, default)]
    #[serde(rename = "githubs")]
    pub r#githubs: Box<Option<Vec<super::types::AccessPolicyExcludeGithub>>>,
    /// The ID of a previously created Access group.
    #[builder(into, default)]
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<String>>>,
    /// Matches a group in Google Workspace. Requires a Google Workspace identity provider.
    #[builder(into, default)]
    #[serde(rename = "gsuites")]
    pub r#gsuites: Box<Option<Vec<super::types::AccessPolicyExcludeGsuite>>>,
    /// The ID of a previously created IP list.
    #[builder(into, default)]
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Box<Option<Vec<String>>>,
    /// An IPv4 or IPv6 CIDR block.
    #[builder(into, default)]
    #[serde(rename = "ips")]
    pub r#ips: Box<Option<Vec<String>>>,
    /// The ID of a configured identity provider.
    #[builder(into, default)]
    #[serde(rename = "loginMethods")]
    pub r#login_methods: Box<Option<Vec<String>>>,
    /// Matches an Okta group. Requires an Okta identity provider.
    #[builder(into, default)]
    #[serde(rename = "oktas")]
    pub r#oktas: Box<Option<Vec<super::types::AccessPolicyExcludeOkta>>>,
    /// Matches a SAML group. Requires a SAML identity provider.
    #[builder(into, default)]
    #[serde(rename = "samls")]
    pub r#samls: Box<Option<Vec<super::types::AccessPolicyExcludeSaml>>>,
    /// The ID of an Access service token.
    #[builder(into, default)]
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Box<Option<Vec<String>>>,
}
