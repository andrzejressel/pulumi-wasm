#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MembershipRbacRoleBindingRole {
    /// PredefinedRole is an ENUM representation of the default Kubernetes Roles
    /// Possible values are: `UNKNOWN`, `ADMIN`, `EDIT`, `VIEW`, `ANTHOS_SUPPORT`.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "predefinedRole")]
    pub r#predefined_role: Box<String>,
}
