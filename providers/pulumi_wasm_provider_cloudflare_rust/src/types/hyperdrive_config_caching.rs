#[derive(serde::Serialize)]
pub struct HyperdriveConfigCaching {
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
}
