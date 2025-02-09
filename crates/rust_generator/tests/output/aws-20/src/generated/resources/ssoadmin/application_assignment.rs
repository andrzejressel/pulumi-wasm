/// Resource for managing an AWS SSO Admin Application Assignment.
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
///     let example = application_assignment::create(
///         "example",
///         ApplicationAssignmentArgs::builder()
///             .application_arn("${exampleAwsSsoadminApplication.applicationArn}")
///             .principal_id("${exampleAwsIdentitystoreUser.userId}")
///             .principal_type("USER")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Group Type
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = application_assignment::create(
///         "example",
///         ApplicationAssignmentArgs::builder()
///             .application_arn("${exampleAwsSsoadminApplication.applicationArn}")
///             .principal_id("${exampleAwsIdentitystoreGroup.groupId}")
///             .principal_type("GROUP")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Admin Application Assignment using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/applicationAssignment:ApplicationAssignment example arn:aws:sso::123456789012:application/id-12345678,abcd1234,USER
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationAssignmentArgs {
        /// ARN of the application.
        #[builder(into)]
        pub application_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An identifier for an object in IAM Identity Center, such as a user or group.
        #[builder(into)]
        pub principal_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Entity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
        #[builder(into)]
        pub principal_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApplicationAssignmentResult {
        /// ARN of the application.
        pub application_arn: pulumi_gestalt_rust::Output<String>,
        /// An identifier for an object in IAM Identity Center, such as a user or group.
        pub principal_id: pulumi_gestalt_rust::Output<String>,
        /// Entity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
        pub principal_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ApplicationAssignmentArgs,
    ) -> ApplicationAssignmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let application_arn_binding_1 = args.application_arn.get_output(context);
        let application_arn_binding = application_arn_binding_1.get_inner();
        let principal_id_binding_1 = args.principal_id.get_output(context);
        let principal_id_binding = principal_id_binding_1.get_inner();
        let principal_type_binding_1 = args.principal_type.get_output(context);
        let principal_type_binding = principal_type_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssoadmin/applicationAssignment:ApplicationAssignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationArn".into(),
                    value: &application_arn_binding,
                },
                register_interface::ObjectField {
                    name: "principalId".into(),
                    value: &principal_id_binding,
                },
                register_interface::ObjectField {
                    name: "principalType".into(),
                    value: &principal_type_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ApplicationAssignmentResult {
            application_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationArn"),
            ),
            principal_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("principalId"),
            ),
            principal_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("principalType"),
            ),
        }
    }
}
