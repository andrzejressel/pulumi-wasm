#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AccessPolicyAssociationAccessScope {
    /// The namespaces to which the access scope applies when type is namespace.
    #[builder(into, default)]
    #[serde(rename = "namespaces")]
    pub r#namespaces: Box<Option<Vec<String>>>,
    /// Valid values are `namespace` or `cluster`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
