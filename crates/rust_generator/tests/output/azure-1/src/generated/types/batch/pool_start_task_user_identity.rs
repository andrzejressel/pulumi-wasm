#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PoolStartTaskUserIdentity {
    /// A `auto_user` block that describes the user identity under which the start task runs as defined below.
    /// 
    /// > **Please Note:** `user_name` and `auto_user` blocks cannot be used both at the same time, but you need to define one or the other.
    #[builder(into, default)]
    #[serde(rename = "autoUser")]
    pub r#auto_user: Box<Option<super::super::types::batch::PoolStartTaskUserIdentityAutoUser>>,
    /// The username to be used by the Batch pool start task.
    #[builder(into, default)]
    #[serde(rename = "userName")]
    pub r#user_name: Box<Option<String>>,
}
