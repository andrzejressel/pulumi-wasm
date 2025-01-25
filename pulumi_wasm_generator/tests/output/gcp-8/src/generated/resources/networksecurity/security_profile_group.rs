/// A security profile group defines a container for security profiles.
///
///
/// To get more information about SecurityProfileGroup, see:
///
/// * [API documentation](https://cloud.google.com/firewall/docs/reference/network-security/rest/v1/organizations.locations.securityProfileGroups)
/// * How-to Guides
///     * [Create and manage security profile groups](https://cloud.google.com/firewall/docs/configure-security-profile-groups)
///     * [Security profile groups overview](https://cloud.google.com/firewall/docs/about-security-profile-groups)
///
/// ## Example Usage
///
/// ### Network Security Security Profile Group Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networksecurity:SecurityProfileGroup
///     properties:
///       name: sec-profile-group
///       parent: organizations/123456789
///       description: my description
///       threatPreventionProfile: ${securityProfile.id}
///       labels:
///         foo: bar
///   securityProfile:
///     type: gcp:networksecurity:SecurityProfile
///     name: security_profile
///     properties:
///       name: sec-profile
///       type: THREAT_PREVENTION
///       parent: organizations/123456789
///       location: global
/// ```
///
/// ## Import
///
/// SecurityProfileGroup can be imported using any of these accepted formats:
///
/// * `{{parent}}/locations/{{location}}/securityProfileGroups/{{name}}`
///
/// When using the `pulumi import` command, SecurityProfileGroup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networksecurity/securityProfileGroup:SecurityProfileGroup default {{parent}}/locations/{{location}}/securityProfileGroups/{{name}}
/// ```
///
pub mod security_profile_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityProfileGroupArgs {
        /// An optional description of the profile. The Max length is 512 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of key/value label pairs to assign to the resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the security profile group.
        /// The default value is `global`.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the security profile group resource.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the parent this security profile group belongs to.
        /// Format: organizations/{organization_id}.
        #[builder(into, default)]
        pub parent: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Reference to a SecurityProfile with the threat prevention configuration for the SecurityProfileGroup.
        #[builder(into, default)]
        pub threat_prevention_profile: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SecurityProfileGroupResult {
        /// Time the security profile group was created in UTC.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// An optional description of the profile. The Max length is 512 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// This checksum is computed by the server based on the value of other fields,
        /// and may be sent on update and delete requests to ensure the client has an up-to-date
        /// value before proceeding.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// A map of key/value label pairs to assign to the resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the security profile group.
        /// The default value is `global`.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the security profile group resource.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the parent this security profile group belongs to.
        /// Format: organizations/{organization_id}.
        pub parent: pulumi_wasm_rust::Output<Option<String>>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Reference to a SecurityProfile with the threat prevention configuration for the SecurityProfileGroup.
        pub threat_prevention_profile: pulumi_wasm_rust::Output<Option<String>>,
        /// Time the security profile group was updated in UTC.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SecurityProfileGroupArgs,
    ) -> SecurityProfileGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let threat_prevention_profile_binding = args
            .threat_prevention_profile
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networksecurity/securityProfileGroup:SecurityProfileGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "threatPreventionProfile".into(),
                    value: &threat_prevention_profile_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "threatPreventionProfile".into(),
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
        SecurityProfileGroupResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            threat_prevention_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("threatPreventionProfile").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
