#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LustreFileSystemRootSquashConfiguration {
    /// When root squash is enabled, you can optionally specify an array of NIDs of clients for which root squash does not apply. A client NID is a Lustre Network Identifier used to uniquely identify a client. You can specify the NID as either a single address or a range of addresses: 1. A single address is described in standard Lustre NID format by specifying the client’s IP address followed by the Lustre network ID (for example, 10.0.1.6@tcp). 2. An address range is described using a dash to separate the range (for example, 10.0.[2-10].[1-255]@tcp).
    #[builder(into, default)]
    #[serde(rename = "noSquashNids")]
    pub r#no_squash_nids: Box<Option<Vec<String>>>,
    /// You enable root squash by setting a user ID (UID) and group ID (GID) for the file system in the format UID:GID (for example, 365534:65534). The UID and GID values can range from 0 to 4294967294.
    #[builder(into, default)]
    #[serde(rename = "rootSquash")]
    pub r#root_squash: Box<Option<String>>,
}
