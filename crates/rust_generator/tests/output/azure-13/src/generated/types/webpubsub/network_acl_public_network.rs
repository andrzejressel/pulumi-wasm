#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkAclPublicNetwork {
    /// The allowed request types for the public network. Possible values are `ClientConnection`, `ServerConnection`, `RESTAPI` and `Trace`.
    #[builder(into, default)]
    #[serde(rename = "allowedRequestTypes")]
    pub r#allowed_request_types: Box<Option<Vec<String>>>,
    /// The denied request types for the public network. Possible values are `ClientConnection`, `ServerConnection`, `RESTAPI` and `Trace`.
    /// 
    /// > **NOTE:** When `default_action` is `Allow`, `allowed_request_types`cannot be set. When `default_action` is `Deny`, `denied_request_types`cannot be set.
    #[builder(into, default)]
    #[serde(rename = "deniedRequestTypes")]
    pub r#denied_request_types: Box<Option<Vec<String>>>,
}
