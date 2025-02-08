#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesCentralServerVirtualMachineDataDisk {
    /// A list of full names of Data Disks per Volume. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "names")]
    pub r#names: Box<Vec<String>>,
    /// The name of the Volume. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** Possible value for Application Server and Central Server is `default`.
    /// 
    /// > **Note:** Possible values for Database Server are `hanaData`, `hanaLog`, `hanaShared` and `usrSap`.
    #[builder(into)]
    #[serde(rename = "volumeName")]
    pub r#volume_name: Box<String>,
}
