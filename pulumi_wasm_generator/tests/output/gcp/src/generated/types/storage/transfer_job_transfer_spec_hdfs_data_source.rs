#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TransferJobTransferSpecHdfsDataSource {
    /// Root path to transfer objects. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should generally not begin with a '/'.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
