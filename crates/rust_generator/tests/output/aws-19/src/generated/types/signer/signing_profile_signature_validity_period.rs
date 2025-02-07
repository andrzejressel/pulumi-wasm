#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SigningProfileSignatureValidityPeriod {
    /// The time unit for signature validity. Valid values: `DAYS`, `MONTHS`, `YEARS`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The numerical value of the time unit for signature validity.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<i32>,
}
