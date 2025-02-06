#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StreamProcessorRegionsOfInterest {
    /// Box representing a region of interest on screen. Only 1 per region is allowed. See `bounding_box`.
    #[builder(into, default)]
    #[serde(rename = "boundingBox")]
    pub r#bounding_box: Box<Option<super::super::types::rekognition::StreamProcessorRegionsOfInterestBoundingBox>>,
    /// Shape made up of up to 10 Point objects to define a region of interest. See `polygon`.
    #[builder(into)]
    #[serde(rename = "polygons")]
    pub r#polygons: Box<Vec<super::super::types::rekognition::StreamProcessorRegionsOfInterestPolygon>>,
}
