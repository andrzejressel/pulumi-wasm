/// Provides a Cloudflare Access Group resource. Access Groups are used
/// in conjunction with Access Policies to restrict access to a
/// particular resource based on group membership.
///
/// > It's required that an `account_id` or `zone_id` is provided and in
///    most cases using either is fine. However, if you're using a scoped
///    access token, you must provide the argument that matches the token's
///    scope. For example, an access token that is scoped to the "example.com"
///    zone needs to use the `zone_id` argument.
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/accessGroup:AccessGroup example <account_id>/<group_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod access_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessGroupArgs {
        /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub excludes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::AccessGroupExclude>>,
        >,
        #[builder(into)]
        pub includes: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::types::AccessGroupInclude>,
        >,
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub requires: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::AccessGroupRequire>>,
        >,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccessGroupResult {
        /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub excludes: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::AccessGroupExclude>>,
        >,
        pub includes: pulumi_gestalt_rust::Output<Vec<super::types::AccessGroupInclude>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub requires: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::AccessGroupRequire>>,
        >,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessGroupArgs,
    ) -> AccessGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let excludes_binding = args.excludes.get_output(context);
        let includes_binding = args.includes.get_output(context);
        let name_binding = args.name.get_output(context);
        let requires_binding = args.requires.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/accessGroup:AccessGroup".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludes".into(),
                    value: excludes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includes".into(),
                    value: includes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requires".into(),
                    value: requires_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccessGroupResult {
            account_id: o.get_field("accountId"),
            excludes: o.get_field("excludes"),
            includes: o.get_field("includes"),
            name: o.get_field("name"),
            requires: o.get_field("requires"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
