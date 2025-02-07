#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionProfilePrivateConnectivity {
    /// A reference to a private connection resource. Format: `projects/{project}/locations/{location}/privateConnections/{name}`
    #[builder(into)]
    #[serde(rename = "privateConnection")]
    pub r#private_connection: Box<String>,
}
