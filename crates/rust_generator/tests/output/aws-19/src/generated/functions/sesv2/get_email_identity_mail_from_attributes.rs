#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_email_identity_mail_from_attributes {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEmailIdentityMailFromAttributesArgs {
        /// The name of the email identity.
        #[builder(into)]
        pub email_identity: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetEmailIdentityMailFromAttributesResult {
        /// The action to take if the required MX record isn't found when you send an email. Valid values: `USE_DEFAULT_VALUE`, `REJECT_MESSAGE`.
        pub behavior_on_mx_failure: pulumi_gestalt_rust::Output<String>,
        pub email_identity: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The custom MAIL FROM domain that you want the verified identity to use.
        pub mail_from_domain: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetEmailIdentityMailFromAttributesArgs,
    ) -> GetEmailIdentityMailFromAttributesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let email_identity_binding = args.email_identity.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:sesv2/getEmailIdentityMailFromAttributes:getEmailIdentityMailFromAttributes"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "emailIdentity".into(),
                    value: &email_identity_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetEmailIdentityMailFromAttributesResult {
            behavior_on_mx_failure: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("behaviorOnMxFailure"),
            ),
            email_identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("emailIdentity"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            mail_from_domain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mailFromDomain"),
            ),
        }
    }
}
