#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketWebsiteConfigurationV2RedirectAllRequestsTo {
    /// Name of the host where requests are redirected.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Box<String>,
    /// Protocol to use when redirecting requests. The default is the protocol that is used in the original request. Valid values: `http`, `https`.
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
}
