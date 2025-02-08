#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RestorePlanRestoreConfigTransformationRuleResourceFilter {
    /// (Filtering parameter) Any resource subject to transformation must
    /// belong to one of the listed "types". If this field is not provided,
    /// no type filtering will be performed
    /// (all resources of all types matching previous filtering parameters
    /// will be candidates for transformation).
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "groupKinds")]
    pub r#group_kinds: Box<Option<Vec<super::super::types::gkebackup::RestorePlanRestoreConfigTransformationRuleResourceFilterGroupKind>>>,
    /// This is a JSONPath expression that matches specific fields of
    /// candidate resources and it operates as a filtering parameter
    /// (resources that are not matched with this expression will not
    /// be candidates for transformation).
    #[builder(into, default)]
    #[serde(rename = "jsonPath")]
    pub r#json_path: Box<Option<String>>,
    /// (Filtering parameter) Any resource subject to transformation must
    /// be contained within one of the listed Kubernetes Namespace in the
    /// Backup. If this field is not provided, no namespace filtering will
    /// be performed (all resources in all Namespaces, including all
    /// cluster-scoped resources, will be candidates for transformation).
    /// To mix cluster-scoped and namespaced resources in the same rule,
    /// use an empty string ("") as one of the target namespaces.
    #[builder(into, default)]
    #[serde(rename = "namespaces")]
    pub r#namespaces: Box<Option<Vec<String>>>,
}
