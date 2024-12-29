/// Resource for managing an AWS SSO Admin Application Access Scope.
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
///     let example = get_instances::invoke(GetInstancesArgs::builder().build_struct());
///     let exampleApplication = application::create(
///         "exampleApplication",
///         ApplicationArgs::builder()
///             .application_provider_arn("arn:aws:sso::aws:applicationProvider/custom")
///             .instance_arn("${example.arns[0]}")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleApplicationAccessScope = application_access_scope::create(
///         "exampleApplicationAccessScope",
///         ApplicationAccessScopeArgs::builder()
///             .application_arn("${exampleApplication.applicationArn}")
///             .authorized_targets(
///                 vec![
///                     "arn:aws:sso::123456789012:application/ssoins-123456789012/apl-123456789012",
///                 ],
///             )
///             .scope("sso:account:access")
///             .build_struct(),
///     );
/// }
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationAccessScopeArgs {
        /// Specifies the ARN of the application with the access scope with the targets to add or update.
        #[builder(into)]
        pub application_arn: pulumi_wasm_rust::Output<String>,
        /// Specifies an array list of ARNs that represent the authorized targets for this access scope.
        #[builder(into, default)]
        pub authorized_targets: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the name of the access scope to be associated with the specified targets.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub scope: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ApplicationAccessScopeResult {
        /// Specifies the ARN of the application with the access scope with the targets to add or update.
        pub application_arn: pulumi_wasm_rust::Output<String>,
        /// Specifies an array list of ARNs that represent the authorized targets for this access scope.
        pub authorized_targets: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the name of the access scope to be associated with the specified targets.
        ///
        /// The following arguments are optional:
        pub scope: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ApplicationAccessScopeArgs,
    ) -> ApplicationAccessScopeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_arn_binding = args.application_arn.get_inner();
        let authorized_targets_binding = args.authorized_targets.get_inner();
        let scope_binding = args.scope.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssoadmin/applicationAccessScope:ApplicationAccessScope".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationArn".into(),
                },
                register_interface::ResultField {
                    name: "authorizedTargets".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApplicationAccessScopeResult {
            application_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationArn").unwrap(),
            ),
            authorized_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizedTargets").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
        }
    }
}
