#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BlockchainNodesEthereumDetailsGethDetails {
    /// Blockchain garbage collection modes. Only applicable when NodeType is FULL or ARCHIVE.
    /// Possible values are: `FULL`, `ARCHIVE`.
    /// 
    /// <a name="nested_additional_endpoints"></a>The `additional_endpoints` block contains:
    #[builder(into, default)]
    #[serde(rename = "garbageCollectionMode")]
    pub r#garbage_collection_mode: Box<Option<String>>,
}
