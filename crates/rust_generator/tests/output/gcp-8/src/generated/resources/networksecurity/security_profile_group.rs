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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod security_profile_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityProfileGroupArgs {
        /// An optional description of the profile. The Max length is 512 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of key/value label pairs to assign to the resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the security profile group.
        /// The default value is `global`.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the security profile group resource.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the parent this security profile group belongs to.
        /// Format: organizations/{organization_id}.
        #[builder(into, default)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Reference to a SecurityProfile with the threat prevention configuration for the SecurityProfileGroup.
        #[builder(into, default)]
        pub threat_prevention_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct SecurityProfileGroupResult {
        /// Time the security profile group was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// An optional description of the profile. The Max length is 512 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// This checksum is computed by the server based on the value of other fields,
        /// and may be sent on update and delete requests to ensure the client has an up-to-date
        /// value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// A map of key/value label pairs to assign to the resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the security profile group.
        /// The default value is `global`.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the security profile group resource.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the parent this security profile group belongs to.
        /// Format: organizations/{organization_id}.
        pub parent: pulumi_gestalt_rust::Output<Option<String>>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Reference to a SecurityProfile with the threat prevention configuration for the SecurityProfileGroup.
        pub threat_prevention_profile: pulumi_gestalt_rust::Output<Option<String>>,
        /// Time the security profile group was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityProfileGroupArgs,
    ) -> SecurityProfileGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let threat_prevention_profile_binding = args
            .threat_prevention_profile
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networksecurity/securityProfileGroup:SecurityProfileGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "threatPreventionProfile".into(),
                    value: threat_prevention_profile_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SecurityProfileGroupResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            etag: o.get_field("etag"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            pulumi_labels: o.get_field("pulumiLabels"),
            threat_prevention_profile: o.get_field("threatPreventionProfile"),
            update_time: o.get_field("updateTime"),
        }
    }
}
