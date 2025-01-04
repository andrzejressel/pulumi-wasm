#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SqlPoolRestore {
    /// Specifies the Snapshot time to restore formatted as an RFC3339 date string. Changing this forces a new Synapse SQL Pool to be created.
    #[builder(into)]
    #[serde(rename = "pointInTime")]
    pub r#point_in_time: Box<String>,
    /// The ID of the Synapse SQL Pool or SQL Database which is to restore. Changing this forces a new Synapse SQL Pool to be created.
    #[builder(into)]
    #[serde(rename = "sourceDatabaseId")]
    pub r#source_database_id: Box<String>,
}
