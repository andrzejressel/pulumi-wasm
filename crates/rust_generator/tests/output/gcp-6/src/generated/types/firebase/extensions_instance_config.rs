#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ExtensionsInstanceConfig {
    /// List of extension events selected by consumer that extension is allowed to
    /// emit, identified by their types.
    #[builder(into, default)]
    #[serde(rename = "allowedEventTypes")]
    pub r#allowed_event_types: Box<Option<Vec<String>>>,
    /// (Output)
    /// The time at which the Extension Instance Config was created.
    #[builder(into, default)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<Option<String>>,
    /// Fully qualified Eventarc resource name that consumers should use for event triggers.
    #[builder(into, default)]
    #[serde(rename = "eventarcChannel")]
    pub r#eventarc_channel: Box<Option<String>>,
    /// The ref of the Extension from the Registry (e.g. publisher-id/awesome-extension)
    #[builder(into)]
    #[serde(rename = "extensionRef")]
    pub r#extension_ref: Box<String>,
    /// The version of the Extension from the Registry (e.g. 1.0.3). If left blank, latest is assumed.
    #[builder(into, default)]
    #[serde(rename = "extensionVersion")]
    pub r#extension_version: Box<Option<String>>,
    /// (Output)
    /// The unique identifier for this configuration.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Environment variables that may be configured for the Extension
    #[builder(into)]
    #[serde(rename = "params")]
    pub r#params: Box<std::collections::HashMap<String, String>>,
    /// (Output)
    /// Postinstall instructions to be shown for this Extension, with
    /// template strings representing function and parameter values substituted
    /// with actual values. These strings include: ${param:FOO},
    /// ${function:myFunc.url},
    /// ${function:myFunc.name}, and ${function:myFunc.location}
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "populatedPostinstallContent")]
    pub r#populated_postinstall_content: Box<Option<String>>,
    /// Params whose values are only available at deployment time.
    /// Unlike other params, these will not be set as environment variables on
    /// functions. See a full list of system parameters at
    /// https://firebase.google.com/docs/extensions/publishers/parameters#system_parameters
    #[builder(into, default)]
    #[serde(rename = "systemParams")]
    pub r#system_params: Box<Option<std::collections::HashMap<String, String>>>,
}
