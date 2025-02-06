#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StreamProcessorRegionsOfInterestPolygon {
    /// The value of the X coordinate for a point on a Polygon.
    #[builder(into, default)]
    #[serde(rename = "x")]
    pub r#x: Box<Option<f64>>,
    /// The value of the Y coordinate for a point on a Polygon.
    #[builder(into, default)]
    #[serde(rename = "y")]
    pub r#y: Box<Option<f64>>,
}
