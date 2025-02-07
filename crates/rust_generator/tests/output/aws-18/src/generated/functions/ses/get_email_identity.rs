pub mod get_email_identity {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEmailIdentityArgs {
        /// Email identity.
        #[builder(into)]
        pub email: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetEmailIdentityResult {
        /// The ARN of the email identity.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Email identity.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetEmailIdentityArgs,
    ) -> GetEmailIdentityResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let email_binding = args.email.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ses/getEmailIdentity:getEmailIdentity".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "email".into(),
                    value: &email_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetEmailIdentityResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            email: pulumi_gestalt_rust::__private::into_domain(o.extract_field("email")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
