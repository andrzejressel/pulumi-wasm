#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LaunchScheduledSplitsConfigStep {
    /// The traffic allocation percentages among the feature variations during one step of a launch. This is a set of key-value pairs. The keys are variation names. The values represent the percentage of traffic to allocate to that variation during this step. For more information, refer to the [AWS documentation for ScheduledSplitConfig groupWeights](https://docs.aws.amazon.com/cloudwatchevidently/latest/APIReference/API_ScheduledSplitConfig.html).
    #[builder(into)]
    #[serde(rename = "groupWeights")]
    pub r#group_weights: Box<std::collections::HashMap<String, i32>>,
    /// One or up to six blocks that specify different traffic splits for one or more audience segments. A segment is a portion of your audience that share one or more characteristics. Examples could be Chrome browser users, users in Europe, or Firefox browser users in Europe who also fit other criteria that your application collects, such as age. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "segmentOverrides")]
    pub r#segment_overrides: Box<Option<Vec<super::super::types::evidently::LaunchScheduledSplitsConfigStepSegmentOverride>>>,
    /// Specifies the date and time that this step of the launch starts.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<String>,
}
