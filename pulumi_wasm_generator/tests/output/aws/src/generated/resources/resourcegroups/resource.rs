/// Resource for managing an AWS Resource Groups Resource.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = dedicated_host::create(
///         "example",
///         DedicatedHostArgs::builder()
///             .auto_placement("on")
///             .availability_zone("us-east-1a")
///             .host_recovery("off")
///             .instance_family("t3")
///             .build_struct(),
///     );
///     let exampleGroup = group::create(
///         "exampleGroup",
///         GroupArgs::builder().name("example").build_struct(),
///     );
///     let exampleResource = resource::create(
///         "exampleResource",
///         ResourceArgs::builder()
///             .group_arn("${exampleGroup.arn}")
///             .resource_arn("${example.arn}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod resource {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceArgs {
        /// The name or the ARN of the resource group to add resources to.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub group_arn: pulumi_wasm_rust::Output<String>,
        /// The ARN of the resource to be added to the group.
        #[builder(into)]
        pub resource_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceResult {
        /// The name or the ARN of the resource group to add resources to.
        ///
        /// The following arguments are optional:
        pub group_arn: pulumi_wasm_rust::Output<String>,
        /// The ARN of the resource to be added to the group.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// The resource type of a resource, such as `AWS::EC2::Instance`.
        pub resource_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ResourceArgs) -> ResourceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let group_arn_binding = args.group_arn.get_inner();
        let resource_arn_binding = args.resource_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:resourcegroups/resource:Resource".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "groupArn".into(),
                    value: &group_arn_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "groupArn".into(),
                },
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
                register_interface::ResultField {
                    name: "resourceType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResourceResult {
            group_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupArn").unwrap(),
            ),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceType").unwrap(),
            ),
        }
    }
}