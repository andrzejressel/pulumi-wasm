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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ApplicationAccessScopeArgs,
    ) -> ApplicationAccessScopeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let application_arn_binding = args
            .application_arn
            .get_output(context)
            .get_inner();
        let authorized_targets_binding = args
            .authorized_targets
            .get_output(context)
            .get_inner();
        let scope_binding = args.scope.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssoadmin/applicationAccessScope:ApplicationAccessScope".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationArn".into(),
                    value: &application_arn_binding,
                },
                register_interface::ObjectField {
                    name: "authorizedTargets".into(),
                    value: &authorized_targets_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ApplicationAccessScopeResult {
            application_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationArn"),
            ),
            authorized_targets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authorizedTargets"),
            ),
            scope: pulumi_gestalt_rust::__private::into_domain(o.extract_field("scope")),
        }
    }
}
