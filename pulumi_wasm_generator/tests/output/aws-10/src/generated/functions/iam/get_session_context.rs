pub mod get_session_context {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSessionContextArgs {
        /// ARN for an assumed role.
        ///
        /// > If `arn` is a non-role ARN, Pulumi gives no error and `issuer_arn` will be equal to the `arn` value. For STS assumed-role ARNs, Pulumi gives an error if the identified IAM role does not exist.
        #[builder(into)]
        pub arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetSessionContextResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// IAM source role ARN if `arn` corresponds to an STS assumed role. Otherwise, `issuer_arn` is equal to `arn`.
        pub issuer_arn: pulumi_wasm_rust::Output<String>,
        /// Unique identifier of the IAM role that issues the STS assumed role.
        pub issuer_id: pulumi_wasm_rust::Output<String>,
        /// Name of the source role. Only available if `arn` corresponds to an STS assumed role.
        pub issuer_name: pulumi_wasm_rust::Output<String>,
        /// Name of the STS session. Only available if `arn` corresponds to an STS assumed role.
        pub session_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSessionContextArgs) -> GetSessionContextResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getSessionContext:getSessionContext".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "issuerArn".into(),
                },
                register_interface::ResultField {
                    name: "issuerId".into(),
                },
                register_interface::ResultField {
                    name: "issuerName".into(),
                },
                register_interface::ResultField {
                    name: "sessionName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSessionContextResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            issuer_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("issuerArn").unwrap(),
            ),
            issuer_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("issuerId").unwrap(),
            ),
            issuer_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("issuerName").unwrap(),
            ),
            session_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sessionName").unwrap(),
            ),
        }
    }
}
