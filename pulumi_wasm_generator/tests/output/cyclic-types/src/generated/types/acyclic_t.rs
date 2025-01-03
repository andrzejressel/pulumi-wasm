#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AcyclicT {
    #[builder(into)]
    #[serde(rename = "foo6")]
    pub r#foo_6: Box<super::types::AcyclicS>,
}
