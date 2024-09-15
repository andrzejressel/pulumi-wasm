#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct AccessGroupRequire {
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Box<Option<bool>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Box<Option<Vec<crate::types::AccessGroupRequireAuthContext>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "authMethod")]
    pub r#auth_method: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "azures")]
    pub r#azures: Box<Option<Vec<crate::types::AccessGroupRequireAzure>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "certificate")]
    pub r#certificate: Box<Option<bool>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "commonName")]
    pub r#common_name: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "devicePostures")]
    pub r#device_postures: Box<Option<Vec<String>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "emailDomains")]
    pub r#email_domains: Box<Option<Vec<String>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "everyone")]
    pub r#everyone: Box<Option<bool>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "externalEvaluation")]
    pub r#external_evaluation: Box<Option<crate::types::AccessGroupRequireExternalEvaluation>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "geos")]
    pub r#geos: Box<Option<Vec<String>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "githubs")]
    pub r#githubs: Box<Option<Vec<crate::types::AccessGroupRequireGithub>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<String>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "gsuites")]
    pub r#gsuites: Box<Option<Vec<crate::types::AccessGroupRequireGsuite>>>,
    /// The ID of an existing IP list to reference.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Box<Option<Vec<String>>>,
    /// An IPv4 or IPv6 CIDR block.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "ips")]
    pub r#ips: Box<Option<Vec<String>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "loginMethods")]
    pub r#login_methods: Box<Option<Vec<String>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "oktas")]
    pub r#oktas: Box<Option<Vec<crate::types::AccessGroupRequireOkta>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "samls")]
    pub r#samls: Box<Option<Vec<crate::types::AccessGroupRequireSaml>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Box<Option<Vec<String>>>,
}
