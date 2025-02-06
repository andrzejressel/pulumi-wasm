#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxPageFormParameterFillBehaviorRepromptEventHandlerTriggerFulfillmentMessageTelephonyTransferCall {
    /// Transfer the call to a phone number in E.164 format.
    #[builder(into)]
    #[serde(rename = "phoneNumber")]
    pub r#phone_number: Box<String>,
}
