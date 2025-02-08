#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServicePerimeterStatusEgressPolicyEgressFromSource {
    /// An AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside.
    #[builder(into, default)]
    #[serde(rename = "accessLevel")]
    pub r#access_level: Box<Option<String>>,
}
