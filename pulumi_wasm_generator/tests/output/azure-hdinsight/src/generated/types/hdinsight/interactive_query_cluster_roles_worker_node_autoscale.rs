#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InteractiveQueryClusterRolesWorkerNodeAutoscale {
    /// A `recurrence` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "recurrence")]
    pub r#recurrence: Box<Option<super::super::types::hdinsight::InteractiveQueryClusterRolesWorkerNodeAutoscaleRecurrence>>,
}
