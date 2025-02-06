#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BrokerUser {
    /// Whether to enable access to the [ActiveMQ Web Console](http://activemq.apache.org/web-console.html) for the user. Applies to `engine_type` of `ActiveMQ` only.
    #[builder(into, default)]
    #[serde(rename = "consoleAccess")]
    pub r#console_access: Box<Option<bool>>,
    /// List of groups (20 maximum) to which the ActiveMQ user belongs. Applies to `engine_type` of `ActiveMQ` only.
    #[builder(into, default)]
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<String>>>,
    /// Password of the user. It must be 12 to 250 characters long, at least 4 unique characters, and must not contain commas.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// Whether to set set replication user. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "replicationUser")]
    pub r#replication_user: Box<Option<bool>>,
    /// Username of the user.
    /// 
    /// > **NOTE:** AWS currently does not support updating RabbitMQ users. Updates to users can only be in the RabbitMQ UI.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
