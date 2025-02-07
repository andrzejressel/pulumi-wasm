#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetImageApplication {
    /// The app block ARN of the application.
    #[builder(into)]
    #[serde(rename = "appBlockArn")]
    pub r#app_block_arn: Box<String>,
    /// Arn of the image being searched for. Cannot be used with name_regex or name.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// Time at which this image was created.
    #[builder(into)]
    #[serde(rename = "createdTime")]
    pub r#created_time: Box<String>,
    /// Description of image.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// Image name to display.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    /// Bool based on if the application is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// A list named icon_s3_location that contains the following:
    #[builder(into)]
    #[serde(rename = "iconS3Locations")]
    pub r#icon_s_3_locations: Box<Vec<super::super::types::appstream::GetImageApplicationIconS3Location>>,
    /// URL of the application icon. This URL may be time-limited.
    #[builder(into)]
    #[serde(rename = "iconUrl")]
    pub r#icon_url: Box<String>,
    /// List of the instance families of the application.
    #[builder(into)]
    #[serde(rename = "instanceFamilies")]
    pub r#instance_families: Box<Vec<String>>,
    /// Arguments that are passed to the application at it's launch.
    #[builder(into)]
    #[serde(rename = "launchParameters")]
    pub r#launch_parameters: Box<String>,
    /// Path to the application's excecutable in the instance.
    #[builder(into)]
    #[serde(rename = "launchPath")]
    pub r#launch_path: Box<String>,
    /// String to string map that contains additional attributes used to describe the application.
    /// * `Name` - Name of the application.
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: Box<std::collections::HashMap<String, String>>,
    /// Name of the image being searched for. Cannot be used with name_regex or arn.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Array of strings describing the platforms on which the application can run.
    /// Values will be from: WINDOWS | WINDOWS_SERVER_2016 | WINDOWS_SERVER_2019 | WINDOWS_SERVER_2022 | AMAZON_LINUX2
    #[builder(into)]
    #[serde(rename = "platforms")]
    pub r#platforms: Box<Vec<String>>,
    /// Working directory for the application.
    #[builder(into)]
    #[serde(rename = "workingDirectory")]
    pub r#working_directory: Box<String>,
}
