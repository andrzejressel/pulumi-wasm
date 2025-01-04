/// Provides a Cognito Resource Server.
///
/// ## Example Usage
///
/// ### Create a basic resource server
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod resource_server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceServerArgs {
        /// An identifier for the resource server.
        #[builder(into)]
        pub identifier: pulumi_wasm_rust::Output<String>,
        /// A name for the resource server.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of Authorization Scope.
        #[builder(into, default)]
        pub scopes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cognito::ResourceServerScope>>,
        >,
        /// User pool the client belongs to.
        #[builder(into)]
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceServerResult {
        /// An identifier for the resource server.
        pub identifier: pulumi_wasm_rust::Output<String>,
        /// A name for the resource server.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of all scopes configured for this resource server in the format identifier/scope_name.
        pub scope_identifiers: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of Authorization Scope.
        pub scopes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cognito::ResourceServerScope>>,
        >,
        /// User pool the client belongs to.
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ResourceServerArgs) -> ResourceServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identifier_binding = args.identifier.get_inner();
        let name_binding = args.name.get_inner();
        let scopes_binding = args.scopes.get_inner();
        let user_pool_id_binding = args.user_pool_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cognito/resourceServer:ResourceServer".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "identifier".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "scopeIdentifiers".into(),
                },
                register_interface::ResultField {
                    name: "scopes".into(),
                },
                register_interface::ResultField {
                    name: "userPoolId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResourceServerResult {
            identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identifier").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            scope_identifiers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scopeIdentifiers").unwrap(),
            ),
            scopes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scopes").unwrap(),
            ),
            user_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userPoolId").unwrap(),
            ),
        }
    }
}
