#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ExperimentStepBranchAction {
    /// The type of action that should be added to the experiment. Possible values are `continuous`, `delay` and `discrete`.
    #[builder(into)]
    #[serde(rename = "actionType")]
    pub r#action_type: Box<String>,
    /// An ISO8601 formatted string specifying the duration for a `delay` or `continuous` action.
    #[builder(into, default)]
    #[serde(rename = "duration")]
    pub r#duration: Box<Option<String>>,
    /// A key-value map of additional parameters to configure the action. The values that are accepted by this depend on the `urn` i.e. the capability/fault that is applied. Possible parameter values can be found in this [documentation](https://learn.microsoft.com/azure/chaos-studio/chaos-studio-fault-library)
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<std::collections::HashMap<String, String>>>,
    /// The name of the Selector to which this action should apply to. This must be specified if the `action_type` is `continuous` or `discrete`.
    #[builder(into, default)]
    #[serde(rename = "selectorName")]
    pub r#selector_name: Box<Option<String>>,
    /// The Unique Resource Name of the action, this value is provided by the `azure.chaosstudio.Capability` resource e.g. `azurerm_chaos_studio_capability.example.urn`. This must be specified if the `action_type` is `continuous` or `discrete`.
    #[builder(into, default)]
    #[serde(rename = "urn")]
    pub r#urn: Box<Option<String>>,
}
