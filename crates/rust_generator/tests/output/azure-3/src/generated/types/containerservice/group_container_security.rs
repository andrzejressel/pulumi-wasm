#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GroupContainerSecurity {
    /// Whether the container's permission is elevated to privileged? Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** Currently, this only applies when the `os_type` is `Linux` and the `sku` is `Confidential`.
    #[builder(into)]
    #[serde(rename = "privilegeEnabled")]
    pub r#privilege_enabled: Box<bool>,
}
