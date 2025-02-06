#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformRedactionColor {
    /// The amount of blue in the color as a value in the interval [0, 1].
    #[builder(into, default)]
    #[serde(rename = "blue")]
    pub r#blue: Box<Option<f64>>,
    /// The amount of green in the color as a value in the interval [0, 1].
    #[builder(into, default)]
    #[serde(rename = "green")]
    pub r#green: Box<Option<f64>>,
    /// The amount of red in the color as a value in the interval [0, 1].
    #[builder(into, default)]
    #[serde(rename = "red")]
    pub r#red: Box<Option<f64>>,
}
