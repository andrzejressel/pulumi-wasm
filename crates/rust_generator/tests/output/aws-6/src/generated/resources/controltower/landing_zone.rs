/// Creates a new landing zone using Control Tower. For more information on usage, please see the
/// [AWS Control Tower Landing Zone User Guide](https://docs.aws.amazon.com/controltower/latest/userguide/how-control-tower-works.html).
///
/// ## Import
///
/// Using `pulumi import`, import a Control Tower Landing Zone using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:controltower/landingZone:LandingZone example 1A2B3C4D5E6F7G8H
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod landing_zone {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LandingZoneArgs {
        /// The manifest JSON file is a text file that describes your AWS resources. For examples, review [Launch your landing zone](https://docs.aws.amazon.com/controltower/latest/userguide/lz-api-launch).
        #[builder(into)]
        pub manifest_json: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tags to apply to the landing zone. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The landing zone version.
        #[builder(into)]
        pub version: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LandingZoneResult {
        /// The ARN of the landing zone.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The drift status summary of the landing zone.
        pub drift_statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::controltower::LandingZoneDriftStatus>,
        >,
        /// The latest available version of the landing zone.
        pub latest_available_version: pulumi_gestalt_rust::Output<String>,
        /// The manifest JSON file is a text file that describes your AWS resources. For examples, review [Launch your landing zone](https://docs.aws.amazon.com/controltower/latest/userguide/lz-api-launch).
        pub manifest_json: pulumi_gestalt_rust::Output<String>,
        /// Tags to apply to the landing zone. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the landing zone, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The landing zone version.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LandingZoneArgs,
    ) -> LandingZoneResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let manifest_json_binding_1 = args.manifest_json.get_output(context);
        let manifest_json_binding = manifest_json_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let version_binding_1 = args.version.get_output(context);
        let version_binding = version_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:controltower/landingZone:LandingZone".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "manifestJson".into(),
                    value: &manifest_json_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LandingZoneResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            drift_statuses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("driftStatuses"),
            ),
            latest_available_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("latestAvailableVersion"),
            ),
            manifest_json: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("manifestJson"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
