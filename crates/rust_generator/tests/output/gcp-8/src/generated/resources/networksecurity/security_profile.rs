/// A security profile defines the behavior associated to a profile type.
///
///
/// To get more information about SecurityProfile, see:
///
/// * [API documentation](https://cloud.google.com/firewall/docs/reference/network-security/rest/v1/organizations.locations.securityProfiles)
/// * How-to Guides
///     * [Create and manage security profiles](https://cloud.google.com/firewall/docs/configure-security-profiles)
///
/// ## Example Usage
///
/// ### Network Security Security Profile Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networksecurity:SecurityProfile
///     properties:
///       name: my-security-profile
///       parent: organizations/123456789
///       description: my description
///       type: THREAT_PREVENTION
///       labels:
///         foo: bar
/// ```
/// ### Network Security Security Profile Overrides
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = security_profile::create(
///         "default",
///         SecurityProfileArgs::builder()
///             .description("my description")
///             .name("my-security-profile")
///             .parent("organizations/123456789")
///             .threat_prevention_profile(
///                 SecurityProfileThreatPreventionProfile::builder()
///                     .severityOverrides(
///                         vec![
///                             SecurityProfileThreatPreventionProfileSeverityOverride::builder()
///                             .action("ALLOW").severity("INFORMATIONAL").build_struct(),
///                             SecurityProfileThreatPreventionProfileSeverityOverride::builder()
///                             .action("DENY").severity("HIGH").build_struct(),
///                         ],
///                     )
///                     .threatOverrides(
///                         vec![
///                             SecurityProfileThreatPreventionProfileThreatOverride::builder()
///                             .action("ALLOW").threatId("280647").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .type_("THREAT_PREVENTION")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// SecurityProfile can be imported using any of these accepted formats:
///
/// * `{{parent}}/locations/{{location}}/securityProfiles/{{name}}`
///
/// When using the `pulumi import` command, SecurityProfile can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networksecurity/securityProfile:SecurityProfile default {{parent}}/locations/{{location}}/securityProfiles/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod security_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityProfileArgs {
        /// An optional description of the security profile. The Max length is 512 characters.
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
        /// The location of the security profile.
        /// The default value is `global`.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the security profile resource.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the parent this security profile belongs to.
        /// Format: organizations/{organization_id}.
        #[builder(into, default)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The threat prevention configuration for the security profile.
        /// Structure is documented below.
        #[builder(into, default)]
        pub threat_prevention_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::networksecurity::SecurityProfileThreatPreventionProfile,
            >,
        >,
        /// The type of security profile.
        /// Possible values are: `THREAT_PREVENTION`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SecurityProfileResult {
        /// Time the security profile was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// An optional description of the security profile. The Max length is 512 characters.
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
        /// The location of the security profile.
        /// The default value is `global`.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the security profile resource.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the parent this security profile belongs to.
        /// Format: organizations/{organization_id}.
        pub parent: pulumi_gestalt_rust::Output<Option<String>>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Server-defined URL of this resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// The threat prevention configuration for the security profile.
        /// Structure is documented below.
        pub threat_prevention_profile: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::networksecurity::SecurityProfileThreatPreventionProfile,
            >,
        >,
        /// The type of security profile.
        /// Possible values are: `THREAT_PREVENTION`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Time the security profile was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityProfileArgs,
    ) -> SecurityProfileResult {
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
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networksecurity/securityProfile:SecurityProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: &parent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "threatPreventionProfile".into(),
                    value: &threat_prevention_profile_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SecurityProfileResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            etag: o.get_field("etag"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            pulumi_labels: o.get_field("pulumiLabels"),
            self_link: o.get_field("selfLink"),
            threat_prevention_profile: o.get_field("threatPreventionProfile"),
            type_: o.get_field("type"),
            update_time: o.get_field("updateTime"),
        }
    }
}
