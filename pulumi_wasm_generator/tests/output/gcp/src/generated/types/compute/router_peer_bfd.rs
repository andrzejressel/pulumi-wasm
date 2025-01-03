#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RouterPeerBfd {
    /// The minimum interval, in milliseconds, between BFD control packets
    /// received from the peer router. The actual value is negotiated
    /// between the two routers and is equal to the greater of this value
    /// and the transmit interval of the other router. If set, this value
    /// must be between 1000 and 30000.
    #[builder(into, default)]
    #[serde(rename = "minReceiveInterval")]
    pub r#min_receive_interval: Box<Option<i32>>,
    /// The minimum interval, in milliseconds, between BFD control packets
    /// transmitted to the peer router. The actual value is negotiated
    /// between the two routers and is equal to the greater of this value
    /// and the corresponding receive interval of the other router. If set,
    /// this value must be between 1000 and 30000.
    #[builder(into, default)]
    #[serde(rename = "minTransmitInterval")]
    pub r#min_transmit_interval: Box<Option<i32>>,
    /// The number of consecutive BFD packets that must be missed before
    /// BFD declares that a peer is unavailable. If set, the value must
    /// be a value between 5 and 16.
    /// 
    /// <a name="nested_md5_authentication_key"></a>The `md5_authentication_key` block supports:
    #[builder(into, default)]
    #[serde(rename = "multiplier")]
    pub r#multiplier: Box<Option<i32>>,
    /// The BFD session initialization mode for this BGP peer.
    /// If set to `ACTIVE`, the Cloud Router will initiate the BFD session
    /// for this BGP peer. If set to `PASSIVE`, the Cloud Router will wait
    /// for the peer router to initiate the BFD session for this BGP peer.
    /// If set to `DISABLED`, BFD is disabled for this BGP peer.
    /// Possible values are: `ACTIVE`, `DISABLED`, `PASSIVE`.
    #[builder(into)]
    #[serde(rename = "sessionInitializationMode")]
    pub r#session_initialization_mode: Box<String>,
}
