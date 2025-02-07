#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobTemplateTemplateVolumeEmptyDir {
    /// The different types of medium supported for EmptyDir. Default value: "MEMORY" Possible values: ["MEMORY"]
    #[builder(into)]
    #[serde(rename = "medium")]
    pub r#medium: Box<String>,
    /// Limit on the storage usable by this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. This field's values are of the 'Quantity' k8s type: https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/. The default is nil which means that the limit is undefined. More info: https://kubernetes.io/docs/concepts/storage/volumes/#emptydir.
    #[builder(into)]
    #[serde(rename = "sizeLimit")]
    pub r#size_limit: Box<String>,
}
