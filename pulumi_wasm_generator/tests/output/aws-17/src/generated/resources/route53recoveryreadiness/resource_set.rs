/// Provides an AWS Route 53 Recovery Readiness Resource Set.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_set::create(
///         "example",
///         ResourceSetArgs::builder()
///             .resource_set_name("${[\"my-cw-alarm-set\"]}")
///             .resource_set_type("AWS::CloudWatch::Alarm")
///             .resources(
///                 vec![
///                     ResourceSetResource::builder()
///                     .resourceArn("${exampleAwsCloudwatchMetricAlarm.arn}")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 Recovery Readiness resource set name using the resource set name. For example:
///
/// ```sh
/// $ pulumi import aws:route53recoveryreadiness/resourceSet:ResourceSet my-cw-alarm-set example
/// ```
pub mod resource_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceSetArgs {
        /// Unique name describing the resource set.
        #[builder(into)]
        pub resource_set_name: pulumi_wasm_rust::Output<String>,
        /// Type of the resources in the resource set.
        #[builder(into)]
        pub resource_set_type: pulumi_wasm_rust::Output<String>,
        /// List of resources to add to this resource set. See below.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub resources: pulumi_wasm_rust::Output<
            Vec<super::super::types::route53recoveryreadiness::ResourceSetResource>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResourceSetResult {
        /// ARN of the resource set
        /// * `resources.#.component_id` - Unique identified for DNS Target Resources, use for readiness checks.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Unique name describing the resource set.
        pub resource_set_name: pulumi_wasm_rust::Output<String>,
        /// Type of the resources in the resource set.
        pub resource_set_type: pulumi_wasm_rust::Output<String>,
        /// List of resources to add to this resource set. See below.
        ///
        /// The following arguments are optional:
        pub resources: pulumi_wasm_rust::Output<
            Vec<super::super::types::route53recoveryreadiness::ResourceSetResource>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ResourceSetArgs) -> ResourceSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let resource_set_name_binding = args.resource_set_name.get_inner();
        let resource_set_type_binding = args.resource_set_type.get_inner();
        let resources_binding = args.resources.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53recoveryreadiness/resourceSet:ResourceSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "resourceSetName".into(),
                    value: &resource_set_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceSetType".into(),
                    value: &resource_set_type_binding,
                },
                register_interface::ObjectField {
                    name: "resources".into(),
                    value: &resources_binding,
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
                    name: "resourceSetName".into(),
                },
                register_interface::ResultField {
                    name: "resourceSetType".into(),
                },
                register_interface::ResultField {
                    name: "resources".into(),
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
        ResourceSetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            resource_set_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceSetName").unwrap(),
            ),
            resource_set_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceSetType").unwrap(),
            ),
            resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resources").unwrap(),
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
