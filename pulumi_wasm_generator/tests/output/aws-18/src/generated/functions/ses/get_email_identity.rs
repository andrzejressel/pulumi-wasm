pub mod get_email_identity {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEmailIdentityArgs {
        /// Email identity.
        #[builder(into)]
        pub email: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetEmailIdentityResult {
        /// The ARN of the email identity.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Email identity.
        pub email: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetEmailIdentityArgs) -> GetEmailIdentityResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let email_binding = args.email.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ses/getEmailIdentity:getEmailIdentity".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "email".into(),
                    value: &email_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "email".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetEmailIdentityResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}
