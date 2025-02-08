#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SingleNodeVirtualInstanceSingleServerConfigurationVirtualMachineResourceNamesDataDisk {
    /// A list of full names of Data Disks per Volume. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "names")]
    pub r#names: Box<Vec<String>>,
    /// The name of the Volume. The only possible value is `default`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "volumeName")]
    pub r#volume_name: Box<String>,
}
