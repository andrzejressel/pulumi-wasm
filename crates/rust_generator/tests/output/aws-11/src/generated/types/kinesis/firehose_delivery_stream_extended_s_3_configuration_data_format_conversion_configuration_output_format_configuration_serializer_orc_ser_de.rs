#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationOutputFormatConfigurationSerializerOrcSerDe {
    /// The Hadoop Distributed File System (HDFS) block size. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is 256 MiB and the minimum is 64 MiB. Kinesis Data Firehose uses this value for padding calculations.
    #[builder(into, default)]
    #[serde(rename = "blockSizeBytes")]
    pub r#block_size_bytes: Box<Option<i32>>,
    /// A list of column names for which you want Kinesis Data Firehose to create bloom filters.
    #[builder(into, default)]
    #[serde(rename = "bloomFilterColumns")]
    pub r#bloom_filter_columns: Box<Option<Vec<String>>>,
    /// The Bloom filter false positive probability (FPP). The lower the FPP, the bigger the Bloom filter. The default value is `0.05`, the minimum is `0`, and the maximum is `1`.
    #[builder(into, default)]
    #[serde(rename = "bloomFilterFalsePositiveProbability")]
    pub r#bloom_filter_false_positive_probability: Box<Option<f64>>,
    /// The compression code to use over data blocks. The default is `SNAPPY`.
    #[builder(into, default)]
    #[serde(rename = "compression")]
    pub r#compression: Box<Option<String>>,
    /// A float that represents the fraction of the total number of non-null rows. To turn off dictionary encoding, set this fraction to a number that is less than the number of distinct keys in a dictionary. To always use dictionary encoding, set this threshold to `1`.
    #[builder(into, default)]
    #[serde(rename = "dictionaryKeyThreshold")]
    pub r#dictionary_key_threshold: Box<Option<f64>>,
    /// Set this to `true` to indicate that you want stripes to be padded to the HDFS block boundaries. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is `false`.
    #[builder(into, default)]
    #[serde(rename = "enablePadding")]
    pub r#enable_padding: Box<Option<bool>>,
    /// The version of the file to write. The possible values are `V0_11` and `V0_12`. The default is `V0_12`.
    #[builder(into, default)]
    #[serde(rename = "formatVersion")]
    pub r#format_version: Box<Option<String>>,
    /// A float between 0 and 1 that defines the tolerance for block padding as a decimal fraction of stripe size. The default value is `0.05`, which means 5 percent of stripe size. For the default values of 64 MiB ORC stripes and 256 MiB HDFS blocks, the default block padding tolerance of 5 percent reserves a maximum of 3.2 MiB for padding within the 256 MiB block. In such a case, if the available size within the block is more than 3.2 MiB, a new, smaller stripe is inserted to fit within that space. This ensures that no stripe crosses block boundaries and causes remote reads within a node-local task. Kinesis Data Firehose ignores this parameter when `enable_padding` is `false`.
    #[builder(into, default)]
    #[serde(rename = "paddingTolerance")]
    pub r#padding_tolerance: Box<Option<f64>>,
    /// The number of rows between index entries. The default is `10000` and the minimum is `1000`.
    #[builder(into, default)]
    #[serde(rename = "rowIndexStride")]
    pub r#row_index_stride: Box<Option<i32>>,
    /// The number of bytes in each stripe. The default is 64 MiB and the minimum is 8 MiB.
    #[builder(into, default)]
    #[serde(rename = "stripeSizeBytes")]
    pub r#stripe_size_bytes: Box<Option<i32>>,
}
