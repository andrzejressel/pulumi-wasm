/// Provides a Synthetics Group Association resource.
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
///     let example = group_association::create(
///         "example",
///         GroupAssociationArgs::builder()
///             .canary_arn("${exampleAwsSyntheticsCanary.arn}")
///             .group_name("${exampleAwsSyntheticsGroup.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch Synthetics Group Association using the `canary_arn,group_name`. For example:
///
/// ```sh
/// $ pulumi import aws:synthetics/groupAssociation:GroupAssociation example arn:aws:synthetics:us-west-2:123456789012:canary:tf-acc-test-abcd1234,examplename
/// ```
pub mod group_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupAssociationArgs {
        /// ARN of the canary.
        #[builder(into)]
        pub canary_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the group that the canary will be associated with.
        #[builder(into)]
        pub group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GroupAssociationResult {
        /// ARN of the canary.
        pub canary_arn: pulumi_wasm_rust::Output<String>,
        pub group_arn: pulumi_wasm_rust::Output<String>,
        /// ID of the Group.
        pub group_id: pulumi_wasm_rust::Output<String>,
        /// Name of the group that the canary will be associated with.
        pub group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: GroupAssociationArgs,
    ) -> GroupAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let canary_arn_binding = args.canary_arn.get_output(context).get_inner();
        let group_name_binding = args.group_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:synthetics/groupAssociation:GroupAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "canaryArn".into(),
                    value: &canary_arn_binding,
                },
                register_interface::ObjectField {
                    name: "groupName".into(),
                    value: &group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "canaryArn".into(),
                },
                register_interface::ResultField {
                    name: "groupArn".into(),
                },
                register_interface::ResultField {
                    name: "groupId".into(),
                },
                register_interface::ResultField {
                    name: "groupName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GroupAssociationResult {
            canary_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("canaryArn").unwrap(),
            ),
            group_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupArn").unwrap(),
            ),
            group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupId").unwrap(),
            ),
            group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupName").unwrap(),
            ),
        }
    }
}
