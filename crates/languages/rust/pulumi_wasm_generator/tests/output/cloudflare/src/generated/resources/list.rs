/// ## Example Usage
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/list:List example <account_id>/<list_id>
/// ```
///
pub mod list {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// An optional description of the list.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The items in the list.
        #[builder(into, default)]
        pub items: pulumi_wasm_rust::InputOrOutput<Option<Vec<super::types::ListItem>>>,
        /// The type of items the list will contain. Must provide only one of: `ip`, `redirect`, `hostname`, `asn`..
        #[builder(into)]
        pub kind: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the list.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ListResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// An optional description of the list.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The items in the list.
        pub items: pulumi_wasm_rust::Output<Option<Vec<super::types::ListItem>>>,
        /// The type of items the list will contain. Must provide only one of: `ip`, `redirect`, `hostname`, `asn`..
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The name of the list.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ListArgs,
    ) -> ListResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let items_binding = args.items.get_output(context).get_inner();
        let kind_binding = args.kind.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/list:List".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "items".into(),
                    value: &items_binding,
                },
                register_interface::ObjectField {
                    name: "kind".into(),
                    value: &kind_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ListResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            items: pulumi_wasm_rust::__private::into_domain(o.extract_field("items")),
            kind: pulumi_wasm_rust::__private::into_domain(o.extract_field("kind")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
