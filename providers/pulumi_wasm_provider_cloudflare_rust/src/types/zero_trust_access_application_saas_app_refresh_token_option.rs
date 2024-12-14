#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessApplicationSaasAppRefreshTokenOption {
    /// How long a refresh token will be valid for after creation. Valid units are `m`, `h` and `d`. Must be longer than 1m.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "lifetime")]
    pub r#lifetime: Box<Option<String>>,
}
