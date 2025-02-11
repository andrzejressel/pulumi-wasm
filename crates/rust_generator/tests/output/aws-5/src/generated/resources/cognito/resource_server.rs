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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceServerArgs,
    ) -> ResourceServerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let identifier_binding = args.identifier.get_output(context);
        let name_binding = args.name.get_output(context);
        let scopes_binding = args.scopes.get_output(context);
        let user_pool_id_binding = args.user_pool_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cognito/resourceServer:ResourceServer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identifier".into(),
                    value: &identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopes".into(),
                    value: &scopes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceServerResult {
            identifier: o.get_field("identifier"),
            name: o.get_field("name"),
            scope_identifiers: o.get_field("scopeIdentifiers"),
            scopes: o.get_field("scopes"),
            user_pool_id: o.get_field("userPoolId"),
        }
    }
}
