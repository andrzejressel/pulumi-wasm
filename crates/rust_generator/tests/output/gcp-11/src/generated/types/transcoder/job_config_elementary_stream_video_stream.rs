#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobConfigElementaryStreamVideoStream {
    /// H264 codec settings
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_h264"></a>The `h264` block supports:
    #[builder(into, default)]
    #[serde(rename = "h264")]
    pub r#h_264: Box<Option<super::super::types::transcoder::JobConfigElementaryStreamVideoStreamH264>>,
}
