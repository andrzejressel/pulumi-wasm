#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectionCloudResource {
    /// (Output)
    /// The account ID of the service created for the purpose of this connection.
    #[builder(into, default)]
    #[serde(rename = "serviceAccountId")]
    pub r#service_account_id: Box<Option<String>>,
}
