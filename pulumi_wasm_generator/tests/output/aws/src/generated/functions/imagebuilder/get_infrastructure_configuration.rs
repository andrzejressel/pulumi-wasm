pub mod get_infrastructure_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInfrastructureConfigurationArgs {
        /// ARN of the infrastructure configuration.
        #[builder(into)]
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the infrastructure created by the infrastructure configuration.
        #[builder(into, default)]
        pub resource_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Key-value map of resource tags for the infrastructure configuration.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetInfrastructureConfigurationResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Date the infrastructure configuration was updated.
        pub date_created: pulumi_wasm_rust::Output<String>,
        pub date_updated: pulumi_wasm_rust::Output<String>,
        /// Description of the infrastructure configuration.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Nested list of instance metadata options for the HTTP requests that pipeline builds use to launch EC2 build and test instances.
        pub instance_metadata_options: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetInfrastructureConfigurationInstanceMetadataOption,
            >,
        >,
        /// Name of the IAM Instance Profile associated with the configuration.
        pub instance_profile_name: pulumi_wasm_rust::Output<String>,
        /// Set of EC2 Instance Types associated with the configuration.
        pub instance_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// Name of the EC2 Key Pair associated with the configuration.
        pub key_pair: pulumi_wasm_rust::Output<String>,
        /// Nested list of logging settings.
        pub loggings: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetInfrastructureConfigurationLogging,
            >,
        >,
        /// Name of the infrastructure configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the infrastructure created by the infrastructure configuration.
        pub resource_tags: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of EC2 Security Group identifiers associated with the configuration.
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// ARN of the SNS Topic associated with the configuration.
        pub sns_topic_arn: pulumi_wasm_rust::Output<String>,
        /// Identifier of the EC2 Subnet associated with the configuration.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the infrastructure configuration.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Whether instances are terminated on failure.
        pub terminate_instance_on_failure: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetInfrastructureConfigurationArgs,
    ) -> GetInfrastructureConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_inner();
        let resource_tags_binding = args.resource_tags.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:imagebuilder/getInfrastructureConfiguration:getInfrastructureConfiguration"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "resourceTags".into(),
                    value: &resource_tags_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "dateCreated".into(),
                },
                register_interface::ResultField {
                    name: "dateUpdated".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceMetadataOptions".into(),
                },
                register_interface::ResultField {
                    name: "instanceProfileName".into(),
                },
                register_interface::ResultField {
                    name: "instanceTypes".into(),
                },
                register_interface::ResultField {
                    name: "keyPair".into(),
                },
                register_interface::ResultField {
                    name: "loggings".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceTags".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "snsTopicArn".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "terminateInstanceOnFailure".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetInfrastructureConfigurationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            date_created: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dateCreated").unwrap(),
            ),
            date_updated: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dateUpdated").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_metadata_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceMetadataOptions").unwrap(),
            ),
            instance_profile_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceProfileName").unwrap(),
            ),
            instance_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceTypes").unwrap(),
            ),
            key_pair: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyPair").unwrap(),
            ),
            loggings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggings").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceTags").unwrap(),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIds").unwrap(),
            ),
            sns_topic_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snsTopicArn").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            terminate_instance_on_failure: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("terminateInstanceOnFailure").unwrap(),
            ),
        }
    }
}
