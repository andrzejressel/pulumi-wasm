#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SQuotaPreferenceQuotaConfig {
    /// The annotations map for clients to store small amounts of arbitrary data. Do not put PII or other sensitive information here. See https://google.aip.dev/128#annotations.
    /// An object containing a list of "key: value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`.
    #[builder(into, default)]
    #[serde(rename = "annotations")]
    pub r#annotations: Box<Option<std::collections::HashMap<String, String>>>,
    /// (Output)
    /// Granted quota value.
    #[builder(into, default)]
    #[serde(rename = "grantedValue")]
    pub r#granted_value: Box<Option<String>>,
    /// The preferred value. Must be greater than or equal to -1. If set to -1, it means the value is "unlimited".
    #[builder(into)]
    #[serde(rename = "preferredValue")]
    pub r#preferred_value: Box<String>,
    /// (Output)
    /// The origin of the quota preference request.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "requestOrigin")]
    pub r#request_origin: Box<Option<String>>,
    /// (Output)
    /// Optional details about the state of this quota preference.
    #[builder(into, default)]
    #[serde(rename = "stateDetail")]
    pub r#state_detail: Box<Option<String>>,
    /// (Output)
    /// The trace id that the Google Cloud uses to provision the requested quota. This trace id may be used by the client to contact Cloud support to track the state of a quota preference request. The trace id is only produced for increase requests and is unique for each request. The quota decrease requests do not have a trace id.
    #[builder(into, default)]
    #[serde(rename = "traceId")]
    pub r#trace_id: Box<Option<String>>,
}
