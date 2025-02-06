/// Resource for managing an AWS WorkSpaces Connection Alias.
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
///     let example = connection_alias::create(
///         "example",
///         ConnectionAliasArgs::builder()
///             .connection_string("testdomain.test")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WorkSpaces Connection Alias using the connection alias ID. For example:
///
/// ```sh
/// $ pulumi import aws:workspaces/connectionAlias:ConnectionAlias example rft-8012925589
/// ```
pub mod connection_alias {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionAliasArgs {
        /// The connection string specified for the connection alias. The connection string must be in the form of a fully qualified domain name (FQDN), such as www.example.com.
        #[builder(into)]
        pub connection_string: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of tags assigned to the WorkSpaces Connection Alias. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::workspaces::ConnectionAliasTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectionAliasResult {
        /// The connection string specified for the connection alias. The connection string must be in the form of a fully qualified domain name (FQDN), such as www.example.com.
        pub connection_string: pulumi_wasm_rust::Output<String>,
        /// The identifier of the Amazon Web Services account that owns the connection alias.
        pub owner_account_id: pulumi_wasm_rust::Output<String>,
        /// The current state of the connection alias.
        pub state: pulumi_wasm_rust::Output<String>,
        /// A map of tags assigned to the WorkSpaces Connection Alias. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::workspaces::ConnectionAliasTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ConnectionAliasArgs,
    ) -> ConnectionAliasResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let connection_string_binding = args
            .connection_string
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:workspaces/connectionAlias:ConnectionAlias".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectionString".into(),
                    value: &connection_string_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ConnectionAliasResult {
            connection_string: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectionString"),
            ),
            owner_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ownerAccountId"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
