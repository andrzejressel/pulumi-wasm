/// Resource for managing an AWS SSO Admin Application Assignment Configuration.
///
/// By default, applications will require users to have an explicit assignment in order to access an application.
/// This resource can be used to adjust this default behavior if necessary.
///
/// > Deleting this resource will return the assignment configuration for the application to the default AWS behavior (ie. `assignment_required = true`).
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
///     let example = application_assignment_configuration::create(
///         "example",
///         ApplicationAssignmentConfigurationArgs::builder()
///             .application_arn("${exampleAwsSsoadminApplication.applicationArn}")
///             .assignment_required(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Admin Application Assignment Configuration using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/applicationAssignmentConfiguration:ApplicationAssignmentConfiguration example arn:aws:sso::123456789012:application/id-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application_assignment_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationAssignmentConfigurationArgs {
        /// ARN of the application.
        #[builder(into)]
        pub application_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indicates whether users must have an explicit assignment to access the application. If `false`, all users have access to the application.
        #[builder(into)]
        pub assignment_required: pulumi_gestalt_rust::InputOrOutput<bool>,
    }
    #[allow(dead_code)]
    pub struct ApplicationAssignmentConfigurationResult {
        /// ARN of the application.
        pub application_arn: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether users must have an explicit assignment to access the application. If `false`, all users have access to the application.
        pub assignment_required: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationAssignmentConfigurationArgs,
    ) -> ApplicationAssignmentConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_arn_binding = args.application_arn.get_output(context);
        let assignment_required_binding = args.assignment_required.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssoadmin/applicationAssignmentConfiguration:ApplicationAssignmentConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationArn".into(),
                    value: application_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assignmentRequired".into(),
                    value: assignment_required_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationAssignmentConfigurationResult {
            application_arn: o.get_field("applicationArn"),
            assignment_required: o.get_field("assignmentRequired"),
        }
    }
}
