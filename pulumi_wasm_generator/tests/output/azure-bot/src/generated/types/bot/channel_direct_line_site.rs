#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelDirectLineSite {
    /// Enables/Disables this site. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Is the endpoint parameters enabled for this site?
    #[builder(into, default)]
    #[serde(rename = "endpointParametersEnabled")]
    pub r#endpoint_parameters_enabled: Box<Option<bool>>,
    /// Enables additional security measures for this site, see [Enhanced Directline Authentication Features](https://blog.botframework.com/2018/09/25/enhanced-direct-line-authentication-features). Disabled by default.
    #[builder(into, default)]
    #[serde(rename = "enhancedAuthenticationEnabled")]
    pub r#enhanced_authentication_enabled: Box<Option<bool>>,
    /// Id for the site
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Primary key for accessing this site
    #[builder(into, default)]
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// Secondary key for accessing this site
    #[builder(into, default)]
    #[serde(rename = "key2")]
    pub r#key_2: Box<Option<String>>,
    /// The name of the site
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Is the storage site enabled for detailed logging? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "storageEnabled")]
    pub r#storage_enabled: Box<Option<bool>>,
    /// This field is required when `is_secure_site_enabled` is enabled. Determines which origins can establish a Directline conversation for this site.
    #[builder(into, default)]
    #[serde(rename = "trustedOrigins")]
    pub r#trusted_origins: Box<Option<Vec<String>>>,
    /// Is the user upload enabled for this site? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "userUploadEnabled")]
    pub r#user_upload_enabled: Box<Option<bool>>,
    /// Enables v1 of the Directline protocol for this site. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "v1Allowed")]
    pub r#v_1_allowed: Box<Option<bool>>,
    /// Enables v3 of the Directline protocol for this site. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "v3Allowed")]
    pub r#v_3_allowed: Box<Option<bool>>,
}
