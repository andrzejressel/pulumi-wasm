pub mod get_email_identity {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEmailIdentityArgs {
        /// The name of the email identity.
        #[builder(into)]
        pub email_identity: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetEmailIdentityResult {
        /// ARN of the Email Identity.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub configuration_set_name: pulumi_wasm_rust::Output<String>,
        /// A list of objects that contains at most one element with information about the private key and selector that you want to use to configure DKIM for the identity for Bring Your Own DKIM (BYODKIM) for the identity, or, configures the key length to be used for Easy DKIM.
        pub dkim_signing_attributes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sesv2::GetEmailIdentityDkimSigningAttribute>,
        >,
        pub email_identity: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The email identity type. Valid values: `EMAIL_ADDRESS`, `DOMAIN`.
        pub identity_type: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Specifies whether or not the identity is verified.
        pub verified_for_sending_status: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetEmailIdentityArgs) -> GetEmailIdentityResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let email_identity_binding = args.email_identity.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:sesv2/getEmailIdentity:getEmailIdentity".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "emailIdentity".into(),
                    value: &email_identity_binding,
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
                    name: "configurationSetName".into(),
                },
                register_interface::ResultField {
                    name: "dkimSigningAttributes".into(),
                },
                register_interface::ResultField {
                    name: "emailIdentity".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identityType".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "verifiedForSendingStatus".into(),
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
            configuration_set_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationSetName").unwrap(),
            ),
            dkim_signing_attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dkimSigningAttributes").unwrap(),
            ),
            email_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailIdentity").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identity_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityType").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            verified_for_sending_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("verifiedForSendingStatus").unwrap(),
            ),
        }
    }
}
