pub mod get_email_identity {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEmailIdentityArgs {
        /// The name of the email identity.
        #[builder(into)]
        pub email_identity: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value mapping of resource tags.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetEmailIdentityArgs,
    ) -> GetEmailIdentityResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let email_identity_binding = args.email_identity.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:sesv2/getEmailIdentity:getEmailIdentity".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetEmailIdentityResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            configuration_set_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configurationSetName"),
            ),
            dkim_signing_attributes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dkimSigningAttributes"),
            ),
            email_identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("emailIdentity"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identity_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identityType"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            verified_for_sending_status: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("verifiedForSendingStatus"),
            ),
        }
    }
}
