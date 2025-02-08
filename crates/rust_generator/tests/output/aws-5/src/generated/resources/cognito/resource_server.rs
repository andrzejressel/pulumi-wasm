/// Provides a Cognito Resource Server.
///
/// ## Example Usage
///
/// ### Create a basic resource server
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let pool = user_pool::create(
///         "pool",
///         UserPoolArgs::builder().name("pool").build_struct(),
///     );
///     let resource = resource_server::create(
///         "resource",
///         ResourceServerArgs::builder()
///             .identifier("https://example.com")
///             .name("example")
///             .user_pool_id("${pool.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Create a resource server with sample-scope
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let pool = user_pool::create(
///         "pool",
///         UserPoolArgs::builder().name("pool").build_struct(),
///     );
///     let resource = resource_server::create(
///         "resource",
///         ResourceServerArgs::builder()
///             .identifier("https://example.com")
///             .name("example")
///             .scopes(
///                 vec![
///                     ResourceServerScope::builder()
///                     .scopeDescription("a Sample Scope Description")
///                     .scopeName("sample-scope").build_struct(),
///                 ],
///             )
///             .user_pool_id("${pool.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_cognito_resource_server` using their User Pool ID and Identifier. For example:
///
/// ```sh
/// $ pulumi import aws:cognito/resourceServer:ResourceServer example "us-west-2_abc123|https://example.com"
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod resource_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceServerArgs {
        /// An identifier for the resource server.
        #[builder(into)]
        pub identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A name for the resource server.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of Authorization Scope.
        #[builder(into, default)]
        pub scopes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cognito::ResourceServerScope>>,
        >,
        /// User pool the client belongs to.
        #[builder(into)]
        pub user_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceServerResult {
        /// An identifier for the resource server.
        pub identifier: pulumi_gestalt_rust::Output<String>,
        /// A name for the resource server.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of all scopes configured for this resource server in the format identifier/scope_name.
        pub scope_identifiers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of Authorization Scope.
        pub scopes: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cognito::ResourceServerScope>>,
        >,
        /// User pool the client belongs to.
        pub user_pool_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ResourceServerArgs,
    ) -> ResourceServerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let identifier_binding = args.identifier.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let scopes_binding = args.scopes.get_output(context).get_inner();
        let user_pool_id_binding = args.user_pool_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cognito/resourceServer:ResourceServer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identifier".into(),
                    value: &identifier_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "scopes".into(),
                    value: &scopes_binding,
                },
                register_interface::ObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResourceServerResult {
            identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identifier"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            scope_identifiers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scopeIdentifiers"),
            ),
            scopes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scopes"),
            ),
            user_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userPoolId"),
            ),
        }
    }
}
