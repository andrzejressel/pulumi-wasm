pub mod get_selection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSelectionArgs {
        /// Backup plan ID associated with the selection of resources.
        #[builder(into)]
        pub plan_id: pulumi_wasm_rust::Output<String>,
        /// Backup selection ID.
        #[builder(into)]
        pub selection_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetSelectionResult {
        /// ARN of the IAM role that AWS Backup uses to authenticate when restoring and backing up the target resource. See the [AWS Backup Developer Guide](https://docs.aws.amazon.com/aws-backup/latest/devguide/access-control.html#managed-policies) for additional information about using AWS managed policies or creating custom policies attached to the IAM role.
        pub iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Display name of a resource selection document.
        pub name: pulumi_wasm_rust::Output<String>,
        pub plan_id: pulumi_wasm_rust::Output<String>,
        /// An array of strings that either contain Amazon Resource Names (ARNs) or match patterns of resources to assign to a backup plan..
        pub resources: pulumi_wasm_rust::Output<Vec<String>>,
        pub selection_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSelectionArgs) -> GetSelectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let plan_id_binding = args.plan_id.get_inner();
        let selection_id_binding = args.selection_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:backup/getSelection:getSelection".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "planId".into(),
                    value: &plan_id_binding,
                },
                register_interface::ObjectField {
                    name: "selectionId".into(),
                    value: &selection_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "iamRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "planId".into(),
                },
                register_interface::ResultField {
                    name: "resources".into(),
                },
                register_interface::ResultField {
                    name: "selectionId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSelectionResult {
            iam_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamRoleArn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            plan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("planId").unwrap(),
            ),
            resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resources").unwrap(),
            ),
            selection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selectionId").unwrap(),
            ),
        }
    }
}
