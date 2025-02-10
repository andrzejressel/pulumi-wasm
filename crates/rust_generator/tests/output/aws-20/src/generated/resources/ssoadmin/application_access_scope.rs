/// Resource for managing an AWS SSO Admin Application Access Scope.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   exampleApplication:
///     type: aws:ssoadmin:Application
///     name: example
///     properties:
///       name: example
///       applicationProviderArn: arn:aws:sso::aws:applicationProvider/custom
///       instanceArn: ${example.arns[0]}
///   exampleApplicationAccessScope:
///     type: aws:ssoadmin:ApplicationAccessScope
///     name: example
///     properties:
///       applicationArn: ${exampleApplication.applicationArn}
///       authorizedTargets:
///         - arn:aws:sso::123456789012:application/ssoins-123456789012/apl-123456789012
///       scope: sso:account:access
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ssoadmin:getInstances
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Admin Application Access Scope using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/applicationAccessScope:ApplicationAccessScope example arn:aws:sso::123456789012:application/ssoins-123456789012/apl-123456789012,sso:account:access
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application_access_scope {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationAccessScopeArgs {
        /// Specifies the ARN of the application with the access scope with the targets to add or update.
        #[builder(into)]
        pub application_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies an array list of ARNs that represent the authorized targets for this access scope.
        #[builder(into, default)]
        pub authorized_targets: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the name of the access scope to be associated with the specified targets.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApplicationAccessScopeResult {
        /// Specifies the ARN of the application with the access scope with the targets to add or update.
        pub application_arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies an array list of ARNs that represent the authorized targets for this access scope.
        pub authorized_targets: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the name of the access scope to be associated with the specified targets.
        ///
        /// The following arguments are optional:
        pub scope: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationAccessScopeArgs,
    ) -> ApplicationAccessScopeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_arn_binding = args.application_arn.get_output(context);
        let authorized_targets_binding = args.authorized_targets.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssoadmin/applicationAccessScope:ApplicationAccessScope".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationArn".into(),
                    value: application_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizedTargets".into(),
                    value: authorized_targets_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: scope_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationAccessScopeResult {
            application_arn: o.get_field("applicationArn"),
            authorized_targets: o.get_field("authorizedTargets"),
            scope: o.get_field("scope"),
        }
    }
}
