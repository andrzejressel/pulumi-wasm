/// Provides a Service Discovery Private DNS Namespace resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vpc::create(
///         "example",
///         VpcArgs::builder().cidr_block("10.0.0.0/16").build_struct(),
///     );
///     let examplePrivateDnsNamespace = private_dns_namespace::create(
///         "examplePrivateDnsNamespace",
///         PrivateDnsNamespaceArgs::builder()
///             .description("example")
///             .name("hoge.example.local")
///             .vpc("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Service Discovery Private DNS Namespace using the namespace ID and VPC ID. For example:
///
/// ```sh
/// $ pulumi import aws:servicediscovery/privateDnsNamespace:PrivateDnsNamespace example 0123456789:vpc-123345
/// ```
pub mod private_dns_namespace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrivateDnsNamespaceArgs {
        /// The description that you specify for the namespace when you create it.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the namespace.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the namespace. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of VPC that you want to associate the namespace with.
        #[builder(into)]
        pub vpc: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PrivateDnsNamespaceResult {
        /// The ARN that Amazon Route 53 assigns to the namespace when you create it.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The description that you specify for the namespace when you create it.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID for the hosted zone that Amazon Route 53 creates when you create a namespace.
        pub hosted_zone: pulumi_wasm_rust::Output<String>,
        /// The name of the namespace.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the namespace. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of VPC that you want to associate the namespace with.
        pub vpc: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: PrivateDnsNamespaceArgs,
    ) -> PrivateDnsNamespaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let vpc_binding = args.vpc.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicediscovery/privateDnsNamespace:PrivateDnsNamespace".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpc".into(),
                    value: &vpc_binding,
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
                    name: "hostedZone".into(),
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
                register_interface::ResultField {
                    name: "vpc".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PrivateDnsNamespaceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            hosted_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostedZone").unwrap(),
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
            vpc: pulumi_wasm_rust::__private::into_domain(hashmap.remove("vpc").unwrap()),
        }
    }
}