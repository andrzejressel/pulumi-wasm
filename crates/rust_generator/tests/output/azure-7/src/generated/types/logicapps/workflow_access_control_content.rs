#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowAccessControlContent {
    /// A list of the allowed caller IP address ranges.
    #[builder(into)]
    #[serde(rename = "allowedCallerIpAddressRanges")]
    pub r#allowed_caller_ip_address_ranges: Box<Vec<String>>,
}
