pub mod get_framework {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFrameworkArgs {
        /// Backup framework name.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Tag key-value pair applied to those AWS resources that you want to trigger an evaluation for a rule. A maximum of one key-value pair can be provided.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetFrameworkResult {
        /// ARN of the backup framework.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// One or more control blocks that make up the framework. Each control in the list has a name, input parameters, and scope. Detailed below.
        pub controls: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::backup::GetFrameworkControl>,
        >,
        /// Date and time that a framework is created, in Unix format and Coordinated Universal Time (UTC).
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// Deployment status of a framework. The statuses are: `CREATE_IN_PROGRESS` | `UPDATE_IN_PROGRESS` | `DELETE_IN_PROGRESS` | `COMPLETED`| `FAILED`.
        pub deployment_status: pulumi_wasm_rust::Output<String>,
        /// Description of the framework.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of a parameter, for example, BackupPlanFrequency.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Framework consists of one or more controls. Each control governs a resource, such as backup plans, backup selections, backup vaults, or recovery points. You can also turn AWS Config recording on or off for each resource. The statuses are: `ACTIVE`, `PARTIALLY_ACTIVE`, `INACTIVE`, `UNAVAILABLE`. For more information refer to the [AWS documentation for Framework Status](https://docs.aws.amazon.com/aws-backup/latest/devguide/API_DescribeFramework.html#Backup-DescribeFramework-response-FrameworkStatus)
        pub status: pulumi_wasm_rust::Output<String>,
        /// Tag key-value pair applied to those AWS resources that you want to trigger an evaluation for a rule. A maximum of one key-value pair can be provided.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetFrameworkArgs) -> GetFrameworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:backup/getFramework:getFramework".into(),
            object: Vec::from([
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
                    name: "controls".into(),
                },
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "deploymentStatus".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFrameworkResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            controls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controls").unwrap(),
            ),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            deployment_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentStatus").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
