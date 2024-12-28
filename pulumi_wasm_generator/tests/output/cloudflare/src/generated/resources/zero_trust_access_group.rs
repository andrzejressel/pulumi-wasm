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
/// $ pulumi import cloudflare:index/zeroTrustAccessGroup:ZeroTrustAccessGroup example <account_id>/<group_id>
/// ```
///
pub mod zero_trust_access_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustAccessGroupArgs {
        /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub excludes: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ZeroTrustAccessGroupExclude>>,
        >,
        #[builder(into)]
        pub includes: pulumi_wasm_rust::Output<
            Vec<super::types::ZeroTrustAccessGroupInclude>,
        >,
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub requires: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ZeroTrustAccessGroupRequire>>,
        >,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustAccessGroupResult {
        /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub excludes: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ZeroTrustAccessGroupExclude>>,
        >,
        pub includes: pulumi_wasm_rust::Output<
            Vec<super::types::ZeroTrustAccessGroupInclude>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub requires: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ZeroTrustAccessGroupRequire>>,
        >,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ZeroTrustAccessGroupArgs,
    ) -> ZeroTrustAccessGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let excludes_binding = args.excludes.get_inner();
        let includes_binding = args.includes.get_inner();
        let name_binding = args.name.get_inner();
        let requires_binding = args.requires.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustAccessGroup:ZeroTrustAccessGroup".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "excludes".into(),
                },
                register_interface::ResultField {
                    name: "includes".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "requires".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ZeroTrustAccessGroupResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            excludes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("excludes").unwrap(),
            ),
            includes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includes").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            requires: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requires").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
