#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLocationsLocation {
    /// The friendly name for this location, typically a nearby city name. For example, "Tokyo".
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    /// Cross-service attributes for the location. For example `{"cloud.googleapis.com/region": "us-east1"}`.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Box<std::collections::HashMap<String, String>>,
    /// The canonical id for this location. For example: "us-east1"..
    #[builder(into)]
    #[serde(rename = "locationId")]
    pub r#location_id: Box<String>,
    /// Service-specific metadata. For example the available capacity at the given location.
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: Box<std::collections::HashMap<String, String>>,
    /// Resource name for the location, which may vary between implementations. For example: "projects/example-project/locations/us-east1".
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
