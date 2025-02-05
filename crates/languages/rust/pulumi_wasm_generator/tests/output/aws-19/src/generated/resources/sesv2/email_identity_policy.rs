/// Resource for managing an AWS SESv2 (Simple Email V2) Email Identity Policy.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod email_identity_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailIdentityPolicyArgs {
        /// The email identity.
        #[builder(into)]
        pub email_identity: pulumi_wasm_rust::InputOrOutput<String>,
        /// The text of the policy in JSON format.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the policy.
        #[builder(into)]
        pub policy_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EmailIdentityPolicyResult {
        /// The email identity.
        pub email_identity: pulumi_wasm_rust::Output<String>,
        /// The text of the policy in JSON format.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The name of the policy.
        pub policy_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EmailIdentityPolicyArgs,
    ) -> EmailIdentityPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let email_identity_binding = args.email_identity.get_output(context).get_inner();
        let policy_binding = args.policy.get_output(context).get_inner();
        let policy_name_binding = args.policy_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sesv2/emailIdentityPolicy:EmailIdentityPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "emailIdentity".into(),
                    value: &email_identity_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "policyName".into(),
                    value: &policy_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EmailIdentityPolicyResult {
            email_identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("emailIdentity"),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(o.extract_field("policy")),
            policy_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyName"),
            ),
        }
    }
}
