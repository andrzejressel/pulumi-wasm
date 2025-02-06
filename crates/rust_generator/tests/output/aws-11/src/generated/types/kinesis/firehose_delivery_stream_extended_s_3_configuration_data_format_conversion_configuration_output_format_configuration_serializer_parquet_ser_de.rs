#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationOutputFormatConfigurationSerializerParquetSerDe {
    /// The Hadoop Distributed File System (HDFS) block size. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is 256 MiB and the minimum is 64 MiB. Kinesis Data Firehose uses this value for padding calculations.
    #[builder(into, default)]
    #[serde(rename = "blockSizeBytes")]
    pub r#block_size_bytes: Box<Option<i32>>,
    /// The compression code to use over data blocks. The possible values are `UNCOMPRESSED`, `SNAPPY`, and `GZIP`, with the default being `SNAPPY`. Use `SNAPPY` for higher decompression speed. Use `GZIP` if the compression ratio is more important than speed.
    #[builder(into, default)]
    #[serde(rename = "compression")]
    pub r#compression: Box<Option<String>>,
    /// Indicates whether to enable dictionary compression.
    #[builder(into, default)]
    #[serde(rename = "enableDictionaryCompression")]
    pub r#enable_dictionary_compression: Box<Option<bool>>,
    /// The maximum amount of padding to apply. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is `0`.
    #[builder(into, default)]
    #[serde(rename = "maxPaddingBytes")]
    pub r#max_padding_bytes: Box<Option<i32>>,
    /// The Parquet page size. Column chunks are divided into pages. A page is conceptually an indivisible unit (in terms of compression and encoding). The minimum value is 64 KiB and the default is 1 MiB.
    #[builder(into, default)]
    #[serde(rename = "pageSizeBytes")]
    pub r#page_size_bytes: Box<Option<i32>>,
    /// Indicates the version of row format to output. The possible values are `V1` and `V2`. The default is `V1`.
    #[builder(into, default)]
    #[serde(rename = "writerVersion")]
    pub r#writer_version: Box<Option<String>>,
}
