/// Provides an ElastiCache parameter group resource.
///
/// > **NOTE:** Attempting to remove the `reserved-memory` parameter when `family` is set to `redis2.6` or `redis2.8` may show a perpetual difference in this provider due to an ElastiCache API limitation. Leave that parameter configured with any value to workaround the issue.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = parameter_group::create(
///         "default",
///         ParameterGroupArgs::builder()
///             .family("redis2.8")
///             .name("cache-params")
///             .parameters(
///                 vec![
///                     ParameterGroupParameter::builder().name("activerehashing")
///                     .value("yes").build_struct(), ParameterGroupParameter::builder()
///                     .name("min-slaves-to-write").value("2").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ElastiCache Parameter Groups using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:elasticache/parameterGroup:ParameterGroup default redis-params
/// ```
pub mod parameter_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ParameterGroupArgs {
        /// The description of the ElastiCache parameter group. Defaults to "Managed by Pulumi".
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The family of the ElastiCache parameter group.
        #[builder(into)]
        pub family: pulumi_wasm_rust::Output<String>,
        /// The name of the ElastiCache parameter.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of ElastiCache parameters to apply.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::elasticache::ParameterGroupParameter>>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ParameterGroupResult {
        /// The AWS ARN associated with the parameter group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The description of the ElastiCache parameter group. Defaults to "Managed by Pulumi".
        pub description: pulumi_wasm_rust::Output<String>,
        /// The family of the ElastiCache parameter group.
        pub family: pulumi_wasm_rust::Output<String>,
        /// The name of the ElastiCache parameter.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of ElastiCache parameters to apply.
        pub parameters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::elasticache::ParameterGroupParameter>>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
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
    pub fn create(name: &str, args: ParameterGroupArgs) -> ParameterGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let family_binding = args.family.get_inner();
        let name_binding = args.name.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elasticache/parameterGroup:ParameterGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "family".into(),
                    value: &family_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
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
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "family".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
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
        ParameterGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            family: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("family").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
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