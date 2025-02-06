#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkServicePccRule {
    /// Specifies the name of the rule. This must be unique within the parent service. You must not use any of the following reserved strings - `default`, `requested` or `service`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A precedence value that is used to decide between data flow policy rules when identifying the QoS values to use for a particular SIM. A lower value means a higher priority. This value should be unique among all data flow policy rules configured in the mobile network. Must be between `0` and `255`.
    #[builder(into)]
    #[serde(rename = "precedence")]
    pub r#precedence: Box<i32>,
    /// A `qos_policy` block as defined below. The QoS policy to use for packets matching this rule. If this field is not specified then the Service will define the QoS settings.
    #[builder(into, default)]
    #[serde(rename = "qosPolicy")]
    pub r#qos_policy: Box<Option<super::super::types::mobile::NetworkServicePccRuleQosPolicy>>,
    /// A `service_data_flow_template` block as defined below. The set of service data flow templates to use for this PCC rule.
    #[builder(into)]
    #[serde(rename = "serviceDataFlowTemplates")]
    pub r#service_data_flow_templates: Box<Vec<super::super::types::mobile::NetworkServicePccRuleServiceDataFlowTemplate>>,
    /// Determines whether flows that match this data flow policy rule are permitted. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "trafficControlEnabled")]
    pub r#traffic_control_enabled: Box<Option<bool>>,
}
