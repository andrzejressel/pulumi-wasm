#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OutputServiceBusQueueSerialization {
    /// The encoding of the incoming data in the case of input and the encoding of outgoing data in the case of output. It currently can only be set to `UTF8`.
    /// 
    /// > **NOTE:** This is required when `type` is set to `Csv` or `Json`.
    #[builder(into, default)]
    #[serde(rename = "encoding")]
    pub r#encoding: Box<Option<String>>,
    /// The delimiter that will be used to separate comma-separated value (CSV) records. Possible values are ` ` (space), `,` (comma), `	` (tab), `|` (pipe) and `;`.
    /// 
    /// > **NOTE:** This is required when `type` is set to `Csv`.
    #[builder(into, default)]
    #[serde(rename = "fieldDelimiter")]
    pub r#field_delimiter: Box<Option<String>>,
    /// Specifies the format of the JSON the output will be written in. Possible values are `Array` and `LineSeparated`.
    /// 
    /// > **NOTE:** This is Required and can only be specified when `type` is set to `Json`.
    #[builder(into, default)]
    #[serde(rename = "format")]
    pub r#format: Box<Option<String>>,
    /// The serialization format used for outgoing data streams. Possible values are `Avro`, `Csv`, `Json` and `Parquet`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}