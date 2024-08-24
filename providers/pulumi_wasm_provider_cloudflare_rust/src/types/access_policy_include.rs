#[derive(serde::Serialize)]
pub struct AccessPolicyInclude {
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Box<Option<bool>>,
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Box<Option<Vec<crate::types::AccessPolicyIncludeAuthContext>>>,
    #[serde(rename = "authMethod")]
    pub r#auth_method: Box<Option<String>>,
    #[serde(rename = "azures")]
    pub r#azures: Box<Option<Vec<crate::types::AccessPolicyIncludeAzure>>>,
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<bool>>,
    #[serde(rename = "commonName")]
    pub r#common_name: Box<Option<String>>,
    #[serde(rename = "devicePostures")]
    pub r#device_postures: Box<Option<Vec<String>>>,
    #[serde(rename = "emailDomains")]
    pub r#email_domains: Box<Option<Vec<String>>>,
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    #[serde(rename = "everyone")]
    pub r#everyone: Box<Option<bool>>,
    #[serde(rename = "externalEvaluation")]
    pub r#external_evaluation: Box<Option<crate::types::AccessPolicyIncludeExternalEvaluation>>,
    #[serde(rename = "geos")]
    pub r#geos: Box<Option<Vec<String>>>,
    #[serde(rename = "githubs")]
    pub r#githubs: Box<Option<Vec<crate::types::AccessPolicyIncludeGithub>>>,
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<String>>>,
    #[serde(rename = "gsuites")]
    pub r#gsuites: Box<Option<Vec<crate::types::AccessPolicyIncludeGsuite>>>,
    /// The ID of an existing IP list to reference.
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Box<Option<Vec<String>>>,
    /// An IPv4 or IPv6 CIDR block.
    #[serde(rename = "ips")]
    pub r#ips: Box<Option<Vec<String>>>,
    #[serde(rename = "loginMethods")]
    pub r#login_methods: Box<Option<Vec<String>>>,
    #[serde(rename = "oktas")]
    pub r#oktas: Box<Option<Vec<crate::types::AccessPolicyIncludeOkta>>>,
    #[serde(rename = "samls")]
    pub r#samls: Box<Option<Vec<crate::types::AccessPolicyIncludeSaml>>>,
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Box<Option<Vec<String>>>,
}
