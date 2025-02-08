#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EventTargetEcsTargetCapacityProviderStrategy {
    /// The base value designates how many tasks, at a minimum, to run on the specified capacity provider. Only one capacity provider in a capacity provider strategy can have a base defined. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "base")]
    pub r#base: Box<Option<i32>>,
    /// Short name of the capacity provider.
    #[builder(into)]
    #[serde(rename = "capacityProvider")]
    pub r#capacity_provider: Box<String>,
    /// The weight value designates the relative percentage of the total number of tasks launched that should use the specified capacity provider. The weight value is taken into consideration after the base value, if defined, is satisfied.
    #[builder(into, default)]
    #[serde(rename = "weight")]
    pub r#weight: Box<Option<i32>>,
}
