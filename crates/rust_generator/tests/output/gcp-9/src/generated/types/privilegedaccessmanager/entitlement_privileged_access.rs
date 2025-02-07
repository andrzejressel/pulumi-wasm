#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EntitlementPrivilegedAccess {
    /// GcpIamAccess represents IAM based access control on a GCP resource. Refer to https://cloud.google.com/iam/docs to understand more about IAM.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "gcpIamAccess")]
    pub r#gcp_iam_access: Box<super::super::types::privilegedaccessmanager::EntitlementPrivilegedAccessGcpIamAccess>,
}
