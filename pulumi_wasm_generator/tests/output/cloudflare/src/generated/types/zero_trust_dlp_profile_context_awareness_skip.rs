#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ZeroTrustDlpProfileContextAwarenessSkip {
    /// Return all matches, regardless of context analysis result, if the data is a file.
    #[builder(into)]
    #[serde(rename = "files")]
    pub r#files: Box<bool>,
}
