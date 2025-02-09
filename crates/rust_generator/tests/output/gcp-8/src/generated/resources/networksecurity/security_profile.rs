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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SecurityProfileArgs,
    ) -> SecurityProfileResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let labels_binding_1 = args.labels.get_output(context);
        let labels_binding = labels_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let parent_binding_1 = args.parent.get_output(context);
        let parent_binding = parent_binding_1.get_inner();
        let threat_prevention_profile_binding_1 = args
            .threat_prevention_profile
            .get_output(context);
        let threat_prevention_profile_binding = threat_prevention_profile_binding_1
            .get_inner();
        let type__binding_1 = args.type_.get_output(context);
        let type__binding = type__binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networksecurity/securityProfile:SecurityProfile".into(),
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
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SecurityProfileResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parent"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            threat_prevention_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("threatPreventionProfile"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
