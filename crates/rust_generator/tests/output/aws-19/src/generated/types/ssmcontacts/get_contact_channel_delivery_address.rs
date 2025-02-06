#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetContactChannelDeliveryAddress {
    #[builder(into)]
    #[serde(rename = "simpleAddress")]
    pub r#simple_address: Box<String>,
}
