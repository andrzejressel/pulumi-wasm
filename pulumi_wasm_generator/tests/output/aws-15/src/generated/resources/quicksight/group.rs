/// Resource for managing QuickSight Group
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = group::create(
///         "example",
///         GroupArgs::builder().group_name("tf-example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import QuickSight Group using the aws account id, namespace and group name separated by `/`. For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/group:Group example 123456789123/default/tf-example
/// ```
pub mod group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupArgs {
        /// The ID for the AWS account that the group is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
        #[builder(into, default)]
        pub aws_account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A description for the group.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A name for the group.
        #[builder(into)]
        pub group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The namespace. Currently, you should set this to `default`.
        #[builder(into, default)]
        pub namespace: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GroupResult {
        /// Amazon Resource Name (ARN) of group
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ID for the AWS account that the group is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        /// A description for the group.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A name for the group.
        pub group_name: pulumi_wasm_rust::Output<String>,
        /// The namespace. Currently, you should set this to `default`.
        pub namespace: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: GroupArgs,
    ) -> GroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_account_id_binding = args.aws_account_id.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let group_name_binding = args.group_name.get_output(context).get_inner();
        let namespace_binding = args.namespace.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:quicksight/group:Group".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "groupName".into(),
                    value: &group_name_binding,
                },
                register_interface::ObjectField {
                    name: "namespace".into(),
                    value: &namespace_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "awsAccountId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "groupName".into(),
                },
                register_interface::ResultField {
                    name: "namespace".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsAccountId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupName").unwrap(),
            ),
            namespace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespace").unwrap(),
            ),
        }
    }
}
