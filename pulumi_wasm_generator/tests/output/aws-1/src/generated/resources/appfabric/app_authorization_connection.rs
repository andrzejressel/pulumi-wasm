/// Resource for managing an AWS AppFabric App Authorization Connection.
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
///     let example = app_authorization_connection::create(
///         "example",
///         AppAuthorizationConnectionArgs::builder()
///             .app_authorization_arn("${test.arn}")
///             .app_bundle_arn("${arn}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod app_authorization_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppAuthorizationConnectionArgs {
        /// The Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the app authorization to use for the request.
        #[builder(into)]
        pub app_authorization_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the app bundle to use for the request.
        #[builder(into)]
        pub app_bundle_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Contains OAuth2 authorization information.This is required if the app authorization for the request is configured with an OAuth2 (oauth2) authorization type.
        #[builder(into, default)]
        pub auth_request: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appfabric::AppAuthorizationConnectionAuthRequest>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appfabric::AppAuthorizationConnectionTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct AppAuthorizationConnectionResult {
        /// The name of the application.
        pub app: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) or Universal Unique Identifier (UUID) of the app authorization to use for the request.
        pub app_authorization_arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the app bundle to use for the request.
        pub app_bundle_arn: pulumi_wasm_rust::Output<String>,
        /// Contains OAuth2 authorization information.This is required if the app authorization for the request is configured with an OAuth2 (oauth2) authorization type.
        pub auth_request: pulumi_wasm_rust::Output<
            Option<super::super::types::appfabric::AppAuthorizationConnectionAuthRequest>,
        >,
        /// Contains information about an application tenant, such as the application display name and identifier.
        pub tenants: pulumi_wasm_rust::Output<
            Vec<super::super::types::appfabric::AppAuthorizationConnectionTenant>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::appfabric::AppAuthorizationConnectionTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AppAuthorizationConnectionArgs,
    ) -> AppAuthorizationConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_authorization_arn_binding = args
            .app_authorization_arn
            .get_output(context)
            .get_inner();
        let app_bundle_arn_binding = args.app_bundle_arn.get_output(context).get_inner();
        let auth_request_binding = args.auth_request.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appfabric/appAuthorizationConnection:AppAuthorizationConnection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appAuthorizationArn".into(),
                    value: &app_authorization_arn_binding,
                },
                register_interface::ObjectField {
                    name: "appBundleArn".into(),
                    value: &app_bundle_arn_binding,
                },
                register_interface::ObjectField {
                    name: "authRequest".into(),
                    value: &auth_request_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AppAuthorizationConnectionResult {
            app: pulumi_wasm_rust::__private::into_domain(o.extract_field("app")),
            app_authorization_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appAuthorizationArn"),
            ),
            app_bundle_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appBundleArn"),
            ),
            auth_request: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authRequest"),
            ),
            tenants: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tenants"),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
