#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SingleNodeVirtualInstanceSingleServerConfigurationDiskVolumeConfiguration {
    /// The total number of disks required for the concerned volume. Possible values are at least `1`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "numberOfDisks")]
    pub r#number_of_disks: Box<i32>,
    /// The size of the Disk in GB. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sizeInGb")]
    pub r#size_in_gb: Box<i32>,
    /// The name of the Disk SKU. Possible values are `Premium_LRS`, `PremiumV2_LRS`, `Premium_ZRS`, `Standard_LRS`, `StandardSSD_LRS`, `StandardSSD_ZRS` and `UltraSSD_LRS`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "skuName")]
    pub r#sku_name: Box<String>,
    /// Specifies the volumn name of the database disk. Possible values are `backup`, `hana/data`, `hana/log`, `hana/shared`, `os` and `usr/sap`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "volumeName")]
    pub r#volume_name: Box<String>,
}
