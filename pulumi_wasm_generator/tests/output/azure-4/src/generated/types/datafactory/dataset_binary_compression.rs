#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatasetBinaryCompression {
    /// The level of compression. Possible values are `Fastest` and `Optimal`.
    #[builder(into, default)]
    #[serde(rename = "level")]
    pub r#level: Box<Option<String>>,
    /// The type of compression used during transport. Possible values are `BZip2`, `Deflate`, `GZip`, `Tar`, `TarGZip` and `ZipDeflate`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
