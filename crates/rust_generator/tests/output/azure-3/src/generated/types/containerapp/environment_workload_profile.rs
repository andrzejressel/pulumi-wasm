#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EnvironmentWorkloadProfile {
    /// The maximum number of instances of workload profile that can be deployed in the Container App Environment.
    #[builder(into, default)]
    #[serde(rename = "maximumCount")]
    pub r#maximum_count: Box<Option<i32>>,
    /// The minimum number of instances of workload profile that can be deployed in the Container App Environment.
    #[builder(into, default)]
    #[serde(rename = "minimumCount")]
    pub r#minimum_count: Box<Option<i32>>,
    /// The name of the workload profile.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Workload profile type for the workloads to run on. Possible values include `Consumption`, `D4`, `D8`, `D16`, `D32`, `E4`, `E8`, `E16` and `E32`.
    /// 
    /// > **Note:** A `Consumption` type must have a name of `Consumption` and an environment may only have one `Consumption` Workload Profile.
    /// 
    /// > **Note:** Defining a `Consumption` profile is optional, however, Environments created without an initial Workload Profile cannot have them added at a later time and must be recreated. Similarly, an environment created with Profiles must always have at least one defined Profile, removing all profiles will force a recreation of the resource.
    #[builder(into)]
    #[serde(rename = "workloadProfileType")]
    pub r#workload_profile_type: Box<String>,
}
