#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OptionGroupOption {
    /// List of DB Security Groups for which the option is enabled.
    #[builder(into, default)]
    #[serde(rename = "dbSecurityGroupMemberships")]
    pub r#db_security_group_memberships: Box<Option<Vec<String>>>,
    /// Name of the option (e.g., MEMCACHED).
    #[builder(into)]
    #[serde(rename = "optionName")]
    pub r#option_name: Box<String>,
    /// The option settings to apply. See `option_settings` Block below for more details.
    #[builder(into, default)]
    #[serde(rename = "optionSettings")]
    pub r#option_settings: Box<Option<Vec<super::super::types::rds::OptionGroupOptionOptionSetting>>>,
    /// Port number when connecting to the option (e.g., 11211). Leaving out or removing `port` from your configuration does not remove or clear a port from the option in AWS. AWS may assign a default port. Not including `port` in your configuration means that the AWS provider will ignore a previously set value, a value set by AWS, and any port changes.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// Version of the option (e.g., 13.1.0.0). Leaving out or removing `version` from your configuration does not remove or clear a version from the option in AWS. AWS may assign a default version. Not including `version` in your configuration means that the AWS provider will ignore a previously set value, a value set by AWS, and any version changes.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
    /// List of VPC Security Groups for which the option is enabled.
    #[builder(into, default)]
    #[serde(rename = "vpcSecurityGroupMemberships")]
    pub r#vpc_security_group_memberships: Box<Option<Vec<String>>>,
}
