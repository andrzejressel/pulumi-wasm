#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct BastionShareableLink {
    /// Reference of the virtual machine resource.
    #[builder(into)]
    #[serde(rename = "vm")]
    pub r#vm: Box<String>,
}
