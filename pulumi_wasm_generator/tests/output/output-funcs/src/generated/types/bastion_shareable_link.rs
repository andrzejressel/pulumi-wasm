#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct BastionShareableLink {
    /// Reference of the virtual machine resource.
    #[builder(into)]
    #[serde(rename = "vm")]
    pub r#vm: Box<String>,
}
