/// Resource for managing an AWS SESv2 (Simple Email V2) Email Identity Policy.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = email_identity::create(
///         "example",
///         EmailIdentityArgs::builder().email_identity("testing@example.com").build_struct(),
///     );
///     let exampleEmailIdentityPolicy = email_identity_policy::create(
///         "exampleEmailIdentityPolicy",
///         EmailIdentityPolicyArgs::builder()
///             .email_identity("${example.emailIdentity}")
///             .policy(
///                 "{\n  \"Id\":\"ExampleAuthorizationPolicy\",\n  \"Version\":\"2012-10-17\",\n  \"Statement\":[\n    {\n      \"Sid\":\"AuthorizeIAMUser\",\n      \"Effect\":\"Allow\",\n      \"Resource\":\"${example.arn}\",\n      \"Principal\":{\n        \"AWS\":[\n          \"arn:aws:iam::123456789012:user/John\",\n          \"arn:aws:iam::123456789012:user/Jane\"\n        ]\n      },\n      \"Action\":[\n        \"ses:DeleteEmailIdentity\",\n        \"ses:PutEmailIdentityDkimSigningAttributes\"\n      ]\n    }\n  ]\n}",
///             )
///             .policy_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SESv2 (Simple Email V2) Email Identity Policy using the `example_id_arg`. For example:
///
/// ```sh
/// $ pulumi import aws:sesv2/emailIdentityPolicy:EmailIdentityPolicy example example_email_identity|example_policy_name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod email_identity_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailIdentityPolicyArgs {
        /// The email identity.
        #[builder(into)]
        pub email_identity: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The text of the policy in JSON format.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the policy.
        #[builder(into)]
        pub policy_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EmailIdentityPolicyResult {
        /// The email identity.
        pub email_identity: pulumi_gestalt_rust::Output<String>,
        /// The text of the policy in JSON format.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The name of the policy.
        pub policy_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EmailIdentityPolicyArgs,
    ) -> EmailIdentityPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let email_identity_binding = args.email_identity.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let policy_name_binding = args.policy_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sesv2/emailIdentityPolicy:EmailIdentityPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailIdentity".into(),
                    value: &email_identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyName".into(),
                    value: &policy_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EmailIdentityPolicyResult {
            email_identity: o.get_field("emailIdentity"),
            policy: o.get_field("policy"),
            policy_name: o.get_field("policyName"),
        }
    }
}
