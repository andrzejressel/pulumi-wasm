#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelDestination {
    /// User-specified id. Ths is used in an output group or an output.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Destination settings for a MediaPackage output; one destination for both encoders. See Media Package Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "mediaPackageSettings")]
    pub r#media_package_settings: Box<Option<Vec<super::super::types::medialive::ChannelDestinationMediaPackageSetting>>>,
    /// Destination settings for a Multiplex output; one destination for both encoders. See Multiplex Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "multiplexSettings")]
    pub r#multiplex_settings: Box<Option<super::super::types::medialive::ChannelDestinationMultiplexSettings>>,
    /// Destination settings for a standard output; one destination for each redundant encoder. See Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "settings")]
    pub r#settings: Box<Option<Vec<super::super::types::medialive::ChannelDestinationSetting>>>,
}
