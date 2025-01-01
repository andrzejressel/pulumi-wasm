#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApiKeyRestrictionsAndroidKeyRestrictionsAllowedApplication {
    /// The package name of the application.
    #[builder(into)]
    #[serde(rename = "packageName")]
    pub r#package_name: Box<String>,
    /// The SHA1 fingerprint of the application. For example, both sha1 formats are acceptable : DA:39:A3:EE:5E:6B:4B:0D:32:55:BF:EF:95:60:18:90:AF:D8:07:09 or DA39A3EE5E6B4B0D3255BFEF95601890AFD80709. Output format is the latter.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "sha1Fingerprint")]
    pub r#sha_1_fingerprint: Box<String>,
}
