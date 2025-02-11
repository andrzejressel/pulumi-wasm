#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_email_identity {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEmailIdentityArgs {
        /// The name of the email identity.
        #[builder(into)]
        pub email_identity: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value mapping of resource tags.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetEmailIdentityResult {
        /// ARN of the Email Identity.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub configuration_set_name: pulumi_gestalt_rust::Output<String>,
        /// A list of objects that contains at most one element with information about the private key and selector that you want to use to configure DKIM for the identity for Bring Your Own DKIM (BYODKIM) for the identity, or, configures the key length to be used for Easy DKIM.
        pub dkim_signing_attributes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sesv2::GetEmailIdentityDkimSigningAttribute>,
        >,
        pub email_identity: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The email identity type. Valid values: `EMAIL_ADDRESS`, `DOMAIN`.
        pub identity_type: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Specifies whether or not the identity is verified.
        pub verified_for_sending_status: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetEmailIdentityArgs,
    ) -> GetEmailIdentityResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let email_identity_binding = args.email_identity.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:sesv2/getEmailIdentity:getEmailIdentity".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailIdentity".into(),
                    value: &email_identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetEmailIdentityResult {
            arn: o.get_field("arn"),
            configuration_set_name: o.get_field("configurationSetName"),
            dkim_signing_attributes: o.get_field("dkimSigningAttributes"),
            email_identity: o.get_field("emailIdentity"),
            id: o.get_field("id"),
            identity_type: o.get_field("identityType"),
            tags: o.get_field("tags"),
            verified_for_sending_status: o.get_field("verifiedForSendingStatus"),
        }
    }
}
