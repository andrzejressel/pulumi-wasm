#[allow(clippy::doc_lazy_continuation)]
pub mod get_infrastructure_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInfrastructureConfigurationArgs {
        /// ARN of the infrastructure configuration.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the infrastructure created by the infrastructure configuration.
        #[builder(into, default)]
        pub resource_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Key-value map of resource tags for the infrastructure configuration.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetInfrastructureConfigurationResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Date the infrastructure configuration was updated.
        pub date_created: pulumi_gestalt_rust::Output<String>,
        pub date_updated: pulumi_gestalt_rust::Output<String>,
        /// Description of the infrastructure configuration.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Nested list of instance metadata options for the HTTP requests that pipeline builds use to launch EC2 build and test instances.
        pub instance_metadata_options: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetInfrastructureConfigurationInstanceMetadataOption,
            >,
        >,
        /// Name of the IAM Instance Profile associated with the configuration.
        pub instance_profile_name: pulumi_gestalt_rust::Output<String>,
        /// Set of EC2 Instance Types associated with the configuration.
        pub instance_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Name of the EC2 Key Pair associated with the configuration.
        pub key_pair: pulumi_gestalt_rust::Output<String>,
        /// Nested list of logging settings.
        pub loggings: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetInfrastructureConfigurationLogging,
            >,
        >,
        /// Name of the infrastructure configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the infrastructure created by the infrastructure configuration.
        pub resource_tags: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of EC2 Security Group identifiers associated with the configuration.
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ARN of the SNS Topic associated with the configuration.
        pub sns_topic_arn: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the EC2 Subnet associated with the configuration.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the infrastructure configuration.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Whether instances are terminated on failure.
        pub terminate_instance_on_failure: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetInfrastructureConfigurationArgs,
    ) -> GetInfrastructureConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let resource_tags_binding = args.resource_tags.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:imagebuilder/getInfrastructureConfiguration:getInfrastructureConfiguration"
                .into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetInfrastructureConfigurationResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            date_created: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dateCreated"),
            ),
            date_updated: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dateUpdated"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instance_metadata_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceMetadataOptions"),
            ),
            instance_profile_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceProfileName"),
            ),
            instance_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceTypes"),
            ),
            key_pair: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyPair"),
            ),
            loggings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loggings"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_tags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceTags"),
            ),
            security_group_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroupIds"),
            ),
            sns_topic_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snsTopicArn"),
            ),
            subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            terminate_instance_on_failure: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("terminateInstanceOnFailure"),
            ),
        }
    }
}
