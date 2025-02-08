#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BlockchainNodesEthereumDetails {
    /// (Output)
    /// User-provided key-value pairs
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "additionalEndpoints")]
    pub r#additional_endpoints: Box<Option<Vec<super::super::types::blockchainnodeengine::BlockchainNodesEthereumDetailsAdditionalEndpoint>>>,
    /// Enables JSON-RPC access to functions in the admin namespace. Defaults to false.
    #[builder(into, default)]
    #[serde(rename = "apiEnableAdmin")]
    pub r#api_enable_admin: Box<Option<bool>>,
    /// Enables JSON-RPC access to functions in the debug namespace. Defaults to false.
    #[builder(into, default)]
    #[serde(rename = "apiEnableDebug")]
    pub r#api_enable_debug: Box<Option<bool>>,
    /// The consensus client
    /// Possible values are: `CONSENSUS_CLIENT_UNSPECIFIED`, `LIGHTHOUSE`.
    #[builder(into, default)]
    #[serde(rename = "consensusClient")]
    pub r#consensus_client: Box<Option<String>>,
    /// The execution client
    /// Possible values are: `EXECUTION_CLIENT_UNSPECIFIED`, `GETH`, `ERIGON`.
    #[builder(into, default)]
    #[serde(rename = "executionClient")]
    pub r#execution_client: Box<Option<String>>,
    /// User-provided key-value pairs
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "gethDetails")]
    pub r#geth_details: Box<Option<super::super::types::blockchainnodeengine::BlockchainNodesEthereumDetailsGethDetails>>,
    /// The Ethereum environment being accessed.
    /// Possible values are: `MAINNET`, `TESTNET_GOERLI_PRATER`, `TESTNET_SEPOLIA`.
    #[builder(into, default)]
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    /// The type of Ethereum node.
    /// Possible values are: `LIGHT`, `FULL`, `ARCHIVE`.
    #[builder(into, default)]
    #[serde(rename = "nodeType")]
    pub r#node_type: Box<Option<String>>,
    /// Configuration for validator-related parameters on the beacon client, and for any managed validator client.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "validatorConfig")]
    pub r#validator_config: Box<Option<super::super::types::blockchainnodeengine::BlockchainNodesEthereumDetailsValidatorConfig>>,
}
