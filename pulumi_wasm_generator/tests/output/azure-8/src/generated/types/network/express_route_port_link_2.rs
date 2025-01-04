#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ExpressRoutePortLink2 {
    /// Whether enable administration state on the Express Route Port Link? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "adminEnabled")]
    pub r#admin_enabled: Box<Option<bool>>,
    /// The connector type of the Express Route Port Link.
    #[builder(into, default)]
    #[serde(rename = "connectorType")]
    pub r#connector_type: Box<Option<String>>,
    /// The ID of this Express Route Port Link.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The interface name of the Azure router associated with the Express Route Port Link.
    #[builder(into, default)]
    #[serde(rename = "interfaceName")]
    pub r#interface_name: Box<Option<String>>,
    /// The ID of the Key Vault Secret that contains the Mac security CAK key for this Express Route Port Link.
    #[builder(into, default)]
    #[serde(rename = "macsecCakKeyvaultSecretId")]
    pub r#macsec_cak_keyvault_secret_id: Box<Option<String>>,
    /// The MACSec cipher used for this Express Route Port Link. Possible values are `GcmAes128` and `GcmAes256`. Defaults to `GcmAes128`.
    #[builder(into, default)]
    #[serde(rename = "macsecCipher")]
    pub r#macsec_cipher: Box<Option<String>>,
    /// The ID of the Key Vault Secret that contains the MACSec CKN key for this Express Route Port Link.
    #[builder(into, default)]
    #[serde(rename = "macsecCknKeyvaultSecretId")]
    pub r#macsec_ckn_keyvault_secret_id: Box<Option<String>>,
    /// Should Secure Channel Identifier on the Express Route Port Link be enabled? Defaults to `false`.
    /// 
    /// > **NOTE** `macsec_ckn_keyvault_secret_id` and `macsec_cak_keyvault_secret_id` should be used together with `identity`, so that the Express Route Port instance have the right permission to access the Key Vault.
    #[builder(into, default)]
    #[serde(rename = "macsecSciEnabled")]
    pub r#macsec_sci_enabled: Box<Option<bool>>,
    /// The ID that maps from the Express Route Port Link to the patch panel port.
    #[builder(into, default)]
    #[serde(rename = "patchPanelId")]
    pub r#patch_panel_id: Box<Option<String>>,
    /// The ID that maps from the patch panel port to the rack.
    #[builder(into, default)]
    #[serde(rename = "rackId")]
    pub r#rack_id: Box<Option<String>>,
    /// The name of the Azure router associated with the Express Route Port Link.
    #[builder(into, default)]
    #[serde(rename = "routerName")]
    pub r#router_name: Box<Option<String>>,
}
