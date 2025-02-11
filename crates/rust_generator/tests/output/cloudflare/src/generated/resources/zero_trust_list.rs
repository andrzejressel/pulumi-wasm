/// Provides a Cloudflare Teams List resource. Teams lists are
/// referenced when creating secure web gateway policies or device
/// posture rules.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zero_trust_list::create(
///         "example",
///         ZeroTrustListArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .description("Serial numbers for all corporate devices.")
///             .items(vec!["8GE8721REF", "5RE8543EGG", "1YE2880LNP",])
///             .name("Corporate devices")
///             .type_("SERIAL")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustList:ZeroTrustList example <account_id>/<teams_list_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zero_trust_list {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustListArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the teams list.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The items of the teams list.
        #[builder(into, default)]
        pub items: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The items of the teams list that has explicit description.
        #[builder(into, default)]
        pub items_with_descriptions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ZeroTrustListItemsWithDescription>>,
        >,
        /// Name of the teams list.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The teams list type. Available values: `IP`, `SERIAL`, `URL`, `DOMAIN`, `EMAIL`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustListResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The description of the teams list.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The items of the teams list.
        pub items: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The items of the teams list that has explicit description.
        pub items_with_descriptions: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ZeroTrustListItemsWithDescription>>,
        >,
        /// Name of the teams list.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The teams list type. Available values: `IP`, `SERIAL`, `URL`, `DOMAIN`, `EMAIL`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZeroTrustListArgs,
    ) -> ZeroTrustListResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let items_binding = args.items.get_output(context);
        let items_with_descriptions_binding = args
            .items_with_descriptions
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustList:ZeroTrustList".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "items".into(),
                    value: &items_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "itemsWithDescriptions".into(),
                    value: &items_with_descriptions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZeroTrustListResult {
            account_id: o.get_field("accountId"),
            description: o.get_field("description"),
            items: o.get_field("items"),
            items_with_descriptions: o.get_field("itemsWithDescriptions"),
            name: o.get_field("name"),
            type_: o.get_field("type"),
        }
    }
}
