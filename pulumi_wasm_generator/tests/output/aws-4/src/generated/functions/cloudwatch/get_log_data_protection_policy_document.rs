pub mod get_log_data_protection_policy_document {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLogDataProtectionPolicyDocumentArgs {
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the data protection policy document.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Configures the data protection policy.
        ///
        /// > There must be exactly two statements: the first with an `audit` operation, and the second with a `deidentify` operation.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub statements: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cloudwatch::GetLogDataProtectionPolicyDocumentStatement,
            >,
        >,
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetLogDataProtectionPolicyDocumentResult {
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Standard JSON policy document rendered based on the arguments above.
        pub json: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub statements: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cloudwatch::GetLogDataProtectionPolicyDocumentStatement,
            >,
        >,
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetLogDataProtectionPolicyDocumentArgs,
    ) -> GetLogDataProtectionPolicyDocumentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let statements_binding = args.statements.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudwatch/getLogDataProtectionPolicyDocument:getLogDataProtectionPolicyDocument"
                .into(),
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
                    name: "statements".into(),
                    value: &statements_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "json".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "statements".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLogDataProtectionPolicyDocumentResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("json").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            statements: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statements").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
