#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionPhysicalConnectionRequirements {
    /// The availability zone of the connection. This field is redundant and implied by `subnet_id`, but is currently an api requirement.
    #[builder(into, default)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Box<Option<String>>,
    /// The security group ID list used by the connection.
    #[builder(into, default)]
    #[serde(rename = "securityGroupIdLists")]
    pub r#security_group_id_lists: Box<Option<Vec<String>>>,
    /// The subnet ID used by the connection.
    #[builder(into, default)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<Option<String>>,
}
