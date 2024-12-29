pub mod get_protection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProtectionArgs {
        /// Unique identifier for the protection.
        #[builder(into, default)]
        pub protection_id: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN (Amazon Resource Name) of the resource being protected.
        #[builder(into, default)]
        pub resource_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetProtectionResult {
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of the protection.
        pub name: pulumi_wasm_rust::Output<String>,
        /// ARN of the protection.
        pub protection_arn: pulumi_wasm_rust::Output<String>,
        pub protection_id: pulumi_wasm_rust::Output<String>,
        pub resource_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetProtectionArgs) -> GetProtectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let protection_id_binding = args.protection_id.get_inner();
        let resource_arn_binding = args.resource_arn.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:shield/getProtection:getProtection".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "protectionId".into(),
                    value: &protection_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "protectionArn".into(),
                },
                register_interface::ResultField {
                    name: "protectionId".into(),
                },
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetProtectionResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            protection_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protectionArn").unwrap(),
            ),
            protection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protectionId").unwrap(),
            ),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
        }
    }
}
