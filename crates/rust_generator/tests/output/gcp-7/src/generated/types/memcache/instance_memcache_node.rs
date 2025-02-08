#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceMemcacheNode {
    /// (Output)
    /// Hostname or IP address of the Memcached node used by the clients to connect to the Memcached server on this node.
    #[builder(into, default)]
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
    /// (Output)
    /// Identifier of the Memcached node. The node id does not include project or location like the Memcached instance name.
    #[builder(into, default)]
    #[serde(rename = "nodeId")]
    pub r#node_id: Box<Option<String>>,
    /// (Output)
    /// The port number of the Memcached server on this node.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// (Output)
    /// Current state of the Memcached node.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// (Output)
    /// Location (GCP Zone) for the Memcached node.
    #[builder(into, default)]
    #[serde(rename = "zone")]
    pub r#zone: Box<Option<String>>,
}
