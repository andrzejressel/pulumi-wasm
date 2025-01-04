#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TaskSetServiceRegistries {
    /// The container name value, already specified in the task definition, to be used for your service discovery service.
    #[builder(into, default)]
    #[serde(rename = "containerName")]
    pub r#container_name: Box<Option<String>>,
    /// The port value, already specified in the task definition, to be used for your service discovery service.
    #[builder(into, default)]
    #[serde(rename = "containerPort")]
    pub r#container_port: Box<Option<i32>>,
    /// The port value used if your Service Discovery service specified an SRV record.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// The ARN of the Service Registry. The currently supported service registry is Amazon Route 53 Auto Naming Service(`aws.servicediscovery.Service` resource). For more information, see [Service](https://docs.aws.amazon.com/Route53/latest/APIReference/API_autonaming_Service.html).
    #[builder(into)]
    #[serde(rename = "registryArn")]
    pub r#registry_arn: Box<String>,
}
