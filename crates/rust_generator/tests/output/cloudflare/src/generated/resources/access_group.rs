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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AccessGroupArgs,
    ) -> AccessGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let excludes_binding = args.excludes.get_output(context).get_inner();
        let includes_binding = args.includes.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let requires_binding = args.requires.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/accessGroup:AccessGroup".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "excludes".into(),
                    value: &excludes_binding,
                },
                register_interface::ObjectField {
                    name: "includes".into(),
                    value: &includes_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "requires".into(),
                    value: &requires_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccessGroupResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            excludes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("excludes"),
            ),
            includes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("includes"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            requires: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requires"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
