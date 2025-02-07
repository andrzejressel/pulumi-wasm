#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSharedImagePurchasePlan {
    /// The name of the Shared Image.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// (Optional) The Purchase Plan Product for this Gallery Image.
    #[builder(into)]
    #[serde(rename = "product")]
    pub r#product: Box<String>,
    /// (Optional) The Purchase Plan Publisher for this Gallery Image.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<String>,
}
