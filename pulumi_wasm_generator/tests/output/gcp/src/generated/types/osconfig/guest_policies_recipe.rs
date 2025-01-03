#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GuestPoliciesRecipe {
    /// Resources available to be used in the steps in the recipe.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "artifacts")]
    pub r#artifacts: Box<Option<Vec<super::super::types::osconfig::GuestPoliciesRecipeArtifact>>>,
    /// Default is INSTALLED. The desired state the agent should maintain for this recipe.
    /// INSTALLED: The software recipe is installed on the instance but won't be updated to new versions.
    /// INSTALLED_KEEP_UPDATED: The software recipe is installed on the instance. The recipe is updated to a higher version,
    /// if a higher version of the recipe is assigned to this instance.
    /// REMOVE: Remove is unsupported for software recipes and attempts to create or update a recipe to the REMOVE state is rejected.
    /// Default value is `INSTALLED`.
    /// Possible values are: `INSTALLED`, `UPDATED`, `REMOVED`.
    #[builder(into, default)]
    #[serde(rename = "desiredState")]
    pub r#desired_state: Box<Option<String>>,
    /// Actions to be taken for installing this recipe. On failure it stops executing steps and does not attempt another installation.
    /// Any steps taken (including partially completed steps) are not rolled back.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "installSteps")]
    pub r#install_steps: Box<Option<Vec<super::super::types::osconfig::GuestPoliciesRecipeInstallStep>>>,
    /// Unique identifier for the recipe. Only one recipe with a given name is installed on an instance.
    /// Names are also used to identify resources which helps to determine whether guest policies have conflicts.
    /// This means that requests to create multiple recipes with the same name and version are rejected since they
    /// could potentially have conflicting assignments.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Actions to be taken for updating this recipe. On failure it stops executing steps and does not attempt another update for this recipe.
    /// Any steps taken (including partially completed steps) are not rolled back.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "updateSteps")]
    pub r#update_steps: Box<Option<Vec<super::super::types::osconfig::GuestPoliciesRecipeUpdateStep>>>,
    /// The version of this software recipe. Version can be up to 4 period separated numbers (e.g. 12.34.56.78).
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
