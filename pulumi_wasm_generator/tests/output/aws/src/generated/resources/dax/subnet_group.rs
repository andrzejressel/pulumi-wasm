/// Provides a DAX Subnet Group resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = subnet_group::create(
///         "example",
///         SubnetGroupArgs::builder()
///             .name("example")
///             .subnet_ids(vec!["${example1.id}", "${example2.id}",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DAX Subnet Group using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:dax/subnetGroup:SubnetGroup example my_dax_sg
/// ```
pub mod subnet_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubnetGroupArgs {
        /// A description of the subnet group.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the subnet group.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of VPC subnet IDs for the subnet group.
        #[builder(into)]
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct SubnetGroupResult {
        /// A description of the subnet group.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the subnet group.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of VPC subnet IDs for the subnet group.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// VPC ID of the subnet group.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SubnetGroupArgs) -> SubnetGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let subnet_ids_binding = args.subnet_ids.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dax/subnetGroup:SubnetGroup".into(),
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
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SubnetGroupResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}