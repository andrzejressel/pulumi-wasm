pub mod get_email_identity_mail_from_attributes {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEmailIdentityMailFromAttributesArgs {
        /// The name of the email identity.
        #[builder(into)]
        pub email_identity: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetEmailIdentityMailFromAttributesResult {
        /// The action to take if the required MX record isn't found when you send an email. Valid values: `USE_DEFAULT_VALUE`, `REJECT_MESSAGE`.
        pub behavior_on_mx_failure: pulumi_wasm_rust::Output<String>,
        pub email_identity: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The custom MAIL FROM domain that you want the verified identity to use.
        pub mail_from_domain: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetEmailIdentityMailFromAttributesArgs,
    ) -> GetEmailIdentityMailFromAttributesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let email_identity_binding = args.email_identity.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:sesv2/getEmailIdentityMailFromAttributes:getEmailIdentityMailFromAttributes"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "emailIdentity".into(),
                    value: &email_identity_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "behaviorOnMxFailure".into(),
                },
                register_interface::ResultField {
                    name: "emailIdentity".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "mailFromDomain".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetEmailIdentityMailFromAttributesResult {
            behavior_on_mx_failure: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("behaviorOnMxFailure").unwrap(),
            ),
            email_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailIdentity").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            mail_from_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mailFromDomain").unwrap(),
            ),
        }
    }
}