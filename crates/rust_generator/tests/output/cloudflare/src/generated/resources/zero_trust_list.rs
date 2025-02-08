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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ZeroTrustListArgs,
    ) -> ZeroTrustListResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let items_binding = args.items.get_output(context).get_inner();
        let items_with_descriptions_binding = args
            .items_with_descriptions
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustList:ZeroTrustList".into(),
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
                    name: "itemsWithDescriptions".into(),
                    value: &items_with_descriptions_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZeroTrustListResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            items: pulumi_gestalt_rust::__private::into_domain(o.extract_field("items")),
            items_with_descriptions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("itemsWithDescriptions"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
