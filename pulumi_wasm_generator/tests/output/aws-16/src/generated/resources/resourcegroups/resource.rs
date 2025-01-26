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
///
/// ## Import
///
/// Using `pulumi import`, import an AWS Resource Groups Resource using `group_arn` and `resource_arn`, separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:resourcegroups/resource:Resource example arn:aws:resource-groups:us-west-2:012345678901:group/example,arn:aws:lambda:us-west-2:012345678901:function:example
/// ```
pub mod resource {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceArgs {
        /// Name or ARN of the resource group to add resources to.
        #[builder(into)]
        pub group_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// ARN of the resource to be added to the group.
        #[builder(into)]
        pub resource_arn: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceResult {
        /// Name or ARN of the resource group to add resources to.
        pub group_arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the resource to be added to the group.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// The resource type of a resource, such as `AWS::EC2::Instance`.
        pub resource_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ResourceArgs,
    ) -> ResourceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let group_arn_binding = args.group_arn.get_output(context).get_inner();
        let resource_arn_binding = args.resource_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:resourcegroups/resource:Resource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        let o = register_interface::register(context.get_inner(), &request);
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
