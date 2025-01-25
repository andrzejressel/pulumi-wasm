/// QuotaPreference represents the preferred quota configuration specified for a project, folder or organization. There is only one QuotaPreference resource for a quota value targeting a unique set of dimensions.
///
///
/// To get more information about QuotaPreference, see:
///
/// * [API documentation](https://cloud.google.com/docs/quotas/reference/rest/v1/projects.locations.quotaPreferences)
/// * How-to Guides
///     * [Cloud Quotas Overview](https://cloud.google.com/docs/quotas/overview)
///
/// ## Example Usage
///
/// ### Cloudquotas Quota Preference Basic
///
///
/// ```yaml
/// resources:
///   preference:
///     type: gcp:cloudquota:SQuotaPreference
///     properties:
///       parent: projects/my-project-name
///       name: compute_googleapis_com-CPUS-per-project_us-east1
///       dimensions:
///         region: us-east1
///       service: compute.googleapis.com
///       quotaId: CPUS-per-project-region
///       contactEmail: testuser@gmail.com
///       quotaConfig:
///         preferredValue: 200
/// ```
///
/// ## Import
///
/// QuotaPreference can be imported using any of these accepted formats:
///
/// * `{{parent}}/locations/global/quotaPreferences/{{name}}`
///
/// When using the `pulumi import` command, QuotaPreference can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudquota/sQuotaPreference:SQuotaPreference default {{parent}}/locations/global/quotaPreferences/{{name}}
/// ```
///
pub mod s_quota_preference {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SQuotaPreferenceArgs {
        /// An email address that can be used for quota related communication between the Google Cloud and the user in case the
        /// Google Cloud needs further information to make a decision on whether the user preferred quota can be granted. The Google
        /// account for the email address must have quota update permission for the project, folder or organization this quota
        /// preference is for.
        #[builder(into, default)]
        pub contact_email: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The dimensions that this quota preference applies to. The key of the map entry is the name of a dimension, such as
        /// "region", "zone", "network_id", and the value of the map entry is the dimension value. If a dimension is missing from
        /// the map of dimensions, the quota preference applies to all the dimension values except for those that have other quota
        /// preferences configured for the specific value. NOTE: QuotaPreferences can only be applied across all values of "user"
        /// and "resource" dimension. Do not set values for "user" or "resource" in the dimension map. Example: '{"provider": "Foo
        /// Inc"}' where "provider" is a service specific dimension.
        #[builder(into, default)]
        pub dimensions: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The list of quota safety checks to be ignored. Default value: "QUOTA_SAFETY_CHECK_UNSPECIFIED" Possible values:
        /// ["QUOTA_SAFETY_CHECK_UNSPECIFIED", "QUOTA_DECREASE_BELOW_USAGE", "QUOTA_DECREASE_PERCENTAGE_TOO_HIGH"]
        #[builder(into, default)]
        pub ignore_safety_checks: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The reason / justification for this quota preference.
        #[builder(into, default)]
        pub justification: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The resource name of the quota preference. Required except in the CREATE requests.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The parent of the quota preference. Allowed parents are "projects/[project-id / number]" or "folders/[folder-id / number]" or "organizations/[org-id / number]".
        #[builder(into, default)]
        pub parent: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The preferred quota configuration.
        /// Structure is documented below.
        #[builder(into)]
        pub quota_config: pulumi_wasm_rust::InputOrOutput<
            super::super::types::cloudquota::SQuotaPreferenceQuotaConfig,
        >,
        /// The id of the quota to which the quota preference is applied. A quota id is unique in the service.
        /// Example: `CPUS-per-project-region`.
        #[builder(into, default)]
        pub quota_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the service to which the quota preference is applied.
        #[builder(into, default)]
        pub service: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SQuotaPreferenceResult {
        /// An email address that can be used for quota related communication between the Google Cloud and the user in case the
        /// Google Cloud needs further information to make a decision on whether the user preferred quota can be granted. The Google
        /// account for the email address must have quota update permission for the project, folder or organization this quota
        /// preference is for.
        pub contact_email: pulumi_wasm_rust::Output<Option<String>>,
        /// Create time stamp.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: `2014-10-02T15:01:23Z` and `2014-10-02T15:01:23.045123456Z`.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The dimensions that this quota preference applies to. The key of the map entry is the name of a dimension, such as
        /// "region", "zone", "network_id", and the value of the map entry is the dimension value. If a dimension is missing from
        /// the map of dimensions, the quota preference applies to all the dimension values except for those that have other quota
        /// preferences configured for the specific value. NOTE: QuotaPreferences can only be applied across all values of "user"
        /// and "resource" dimension. Do not set values for "user" or "resource" in the dimension map. Example: '{"provider": "Foo
        /// Inc"}' where "provider" is a service specific dimension.
        pub dimensions: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The current etag of the quota preference. If an etag is provided on update and does not match the current server's etag of the quota preference, the request will be blocked and an ABORTED error will be returned. See https://google.aip.dev/134#etags for more details on etags.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The list of quota safety checks to be ignored. Default value: "QUOTA_SAFETY_CHECK_UNSPECIFIED" Possible values:
        /// ["QUOTA_SAFETY_CHECK_UNSPECIFIED", "QUOTA_DECREASE_BELOW_USAGE", "QUOTA_DECREASE_PERCENTAGE_TOO_HIGH"]
        pub ignore_safety_checks: pulumi_wasm_rust::Output<Option<String>>,
        /// The reason / justification for this quota preference.
        pub justification: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource name of the quota preference. Required except in the CREATE requests.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parent of the quota preference. Allowed parents are "projects/[project-id / number]" or "folders/[folder-id / number]" or "organizations/[org-id / number]".
        pub parent: pulumi_wasm_rust::Output<String>,
        /// The preferred quota configuration.
        /// Structure is documented below.
        pub quota_config: pulumi_wasm_rust::Output<
            super::super::types::cloudquota::SQuotaPreferenceQuotaConfig,
        >,
        /// The id of the quota to which the quota preference is applied. A quota id is unique in the service.
        /// Example: `CPUS-per-project-region`.
        pub quota_id: pulumi_wasm_rust::Output<String>,
        /// Is the quota preference pending Google Cloud approval and fulfillment.
        pub reconciling: pulumi_wasm_rust::Output<bool>,
        /// The name of the service to which the quota preference is applied.
        pub service: pulumi_wasm_rust::Output<String>,
        /// Update time stamp.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: `2014-10-02T15:01:23Z` and `2014-10-02T15:01:23.045123456Z`.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SQuotaPreferenceArgs,
    ) -> SQuotaPreferenceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let contact_email_binding = args.contact_email.get_output(context).get_inner();
        let dimensions_binding = args.dimensions.get_output(context).get_inner();
        let ignore_safety_checks_binding = args
            .ignore_safety_checks
            .get_output(context)
            .get_inner();
        let justification_binding = args.justification.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let quota_config_binding = args.quota_config.get_output(context).get_inner();
        let quota_id_binding = args.quota_id.get_output(context).get_inner();
        let service_binding = args.service.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:cloudquota/sQuotaPreference:SQuotaPreference".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "contactEmail".into(),
                    value: &contact_email_binding,
                },
                register_interface::ObjectField {
                    name: "dimensions".into(),
                    value: &dimensions_binding,
                },
                register_interface::ObjectField {
                    name: "ignoreSafetyChecks".into(),
                    value: &ignore_safety_checks_binding,
                },
                register_interface::ObjectField {
                    name: "justification".into(),
                    value: &justification_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "quotaConfig".into(),
                    value: &quota_config_binding,
                },
                register_interface::ObjectField {
                    name: "quotaId".into(),
                    value: &quota_id_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "contactEmail".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dimensions".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "ignoreSafetyChecks".into(),
                },
                register_interface::ResultField {
                    name: "justification".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "quotaConfig".into(),
                },
                register_interface::ResultField {
                    name: "quotaId".into(),
                },
                register_interface::ResultField {
                    name: "reconciling".into(),
                },
                register_interface::ResultField {
                    name: "service".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SQuotaPreferenceResult {
            contact_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contactEmail").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            dimensions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dimensions").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            ignore_safety_checks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ignoreSafetyChecks").unwrap(),
            ),
            justification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("justification").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            quota_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quotaConfig").unwrap(),
            ),
            quota_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quotaId").unwrap(),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reconciling").unwrap(),
            ),
            service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("service").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
