#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionCommitmentResource {
    /// Name of the accelerator type resource. Applicable only when the type is ACCELERATOR.
    #[builder(into, default)]
    #[serde(rename = "acceleratorType")]
    pub r#accelerator_type: Box<Option<String>>,
    /// The amount of the resource purchased (in a type-dependent unit,
    /// such as bytes). For vCPUs, this can just be an integer. For memory,
    /// this must be provided in MB. Memory must be a multiple of 256 MB,
    /// with up to 6.5GB of memory per every vCPU.
    #[builder(into, default)]
    #[serde(rename = "amount")]
    pub r#amount: Box<Option<String>>,
    /// Type of resource for which this commitment applies.
    /// Possible values are VCPU, MEMORY, LOCAL_SSD, and ACCELERATOR.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
