/// ## Example Usage
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/list:List example <account_id>/<list_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod list {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An optional description of the list.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The items in the list.
        #[builder(into, default)]
        pub items: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ListItem>>,
        >,
        /// The type of items the list will contain. Must provide only one of: `ip`, `redirect`, `hostname`, `asn`..
        #[builder(into)]
        pub kind: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the list.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ListResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// An optional description of the list.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The items in the list.
        pub items: pulumi_gestalt_rust::Output<Option<Vec<super::types::ListItem>>>,
        /// The type of items the list will contain. Must provide only one of: `ip`, `redirect`, `hostname`, `asn`..
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The name of the list.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ListArgs,
    ) -> ListResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let items_binding = args.items.get_output(context);
        let kind_binding = args.kind.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/list:List".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "items".into(),
                    value: items_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kind".into(),
                    value: kind_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ListResult {
            account_id: o.get_field("accountId"),
            description: o.get_field("description"),
            items: o.get_field("items"),
            kind: o.get_field("kind"),
            name: o.get_field("name"),
        }
    }
}
