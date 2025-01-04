#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionJobTriggerInspectJobStorageConfigCloudStorageOptions {
    /// Max number of bytes to scan from a file. If a scanned file's size is bigger than this value
    /// then the rest of the bytes are omitted.
    #[builder(into, default)]
    #[serde(rename = "bytesLimitPerFile")]
    pub r#bytes_limit_per_file: Box<Option<i32>>,
    /// Max percentage of bytes to scan from a file. The rest are omitted. The number of bytes scanned is rounded down.
    /// Must be between 0 and 100, inclusively. Both 0 and 100 means no limit.
    #[builder(into, default)]
    #[serde(rename = "bytesLimitPerFilePercent")]
    pub r#bytes_limit_per_file_percent: Box<Option<i32>>,
    /// Set of files to scan.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fileSet")]
    pub r#file_set: Box<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigCloudStorageOptionsFileSet>,
    /// List of file type groups to include in the scan. If empty, all files are scanned and available data
    /// format processors are applied. In addition, the binary content of the selected files is always scanned as well.
    /// Images are scanned only as binary if the specified region does not support image inspection and no fileTypes were specified.
    /// Each value may be one of: `BINARY_FILE`, `TEXT_FILE`, `IMAGE`, `WORD`, `PDF`, `AVRO`, `CSV`, `TSV`, `POWERPOINT`, `EXCEL`.
    #[builder(into, default)]
    #[serde(rename = "fileTypes")]
    pub r#file_types: Box<Option<Vec<String>>>,
    /// Limits the number of files to scan to this percentage of the input FileSet. Number of files scanned is rounded down.
    /// Must be between 0 and 100, inclusively. Both 0 and 100 means no limit.
    #[builder(into, default)]
    #[serde(rename = "filesLimitPercent")]
    pub r#files_limit_percent: Box<Option<i32>>,
    /// How to sample bytes if not all bytes are scanned. Meaningful only when used in conjunction with bytesLimitPerFile.
    /// If not specified, scanning would start from the top.
    /// Possible values are: `TOP`, `RANDOM_START`.
    #[builder(into, default)]
    #[serde(rename = "sampleMethod")]
    pub r#sample_method: Box<Option<String>>,
}
