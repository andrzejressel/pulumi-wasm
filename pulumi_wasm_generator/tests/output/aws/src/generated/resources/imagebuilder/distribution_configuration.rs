/// Manages an Image Builder Distribution Configuration.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:imagebuilder:DistributionConfiguration
///     properties:
///       name: example
///       distributions:
///         - amiDistributionConfiguration:
///             amiTags:
///               CostCenter: IT
///             name: example-{{ imagebuilder:buildDate }}
///             launchPermission:
///               userIds:
///                 - '123456789012'
///           launchTemplateConfigurations:
///             - launchTemplateId: lt-0aaa1bcde2ff3456
///           region: us-east-1
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_imagebuilder_distribution_configurations` resources using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:imagebuilder/distributionConfiguration:DistributionConfiguration example arn:aws:imagebuilder:us-east-1:123456789012:distribution-configuration/example
/// ```
pub mod distribution_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DistributionConfigurationArgs {
        /// Description of the distribution configuration.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more configuration blocks with distribution settings. Detailed below.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub distributions: pulumi_wasm_rust::Output<
            Vec<super::super::types::imagebuilder::DistributionConfigurationDistribution>,
        >,
        /// Name of the distribution configuration.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags for the distribution configuration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DistributionConfigurationResult {
        /// (Required) Amazon Resource Name (ARN) of the distribution configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Date the distribution configuration was created.
        pub date_created: pulumi_wasm_rust::Output<String>,
        /// Date the distribution configuration was updated.
        pub date_updated: pulumi_wasm_rust::Output<String>,
        /// Description of the distribution configuration.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more configuration blocks with distribution settings. Detailed below.
        ///
        /// The following arguments are optional:
        pub distributions: pulumi_wasm_rust::Output<
            Vec<super::super::types::imagebuilder::DistributionConfigurationDistribution>,
        >,
        /// Name of the distribution configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the distribution configuration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DistributionConfigurationArgs,
    ) -> DistributionConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let distributions_binding = args.distributions.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:imagebuilder/distributionConfiguration:DistributionConfiguration"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "distributions".into(),
                    value: &distributions_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "distributions".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DistributionConfigurationResult {
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
            distributions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("distributions").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
