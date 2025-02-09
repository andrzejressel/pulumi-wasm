/// Provides a resource to manage a GuardDuty ThreatIntelSet.
///
/// > **Note:** Currently in GuardDuty, users from member accounts cannot upload and further manage ThreatIntelSets. ThreatIntelSets that are uploaded by the primary account are imposed on GuardDuty functionality in its member accounts. See the [GuardDuty API Documentation](https://docs.aws.amazon.com/guardduty/latest/ug/create-threat-intel-set.html)
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bucket = bucket_v_2::create("bucket", BucketV2Args::builder().build_struct());
///     let bucketAcl = bucket_acl_v_2::create(
///         "bucketAcl",
///         BucketAclV2Args::builder().acl("private").bucket("${bucket.id}").build_struct(),
///     );
///     let myThreatIntelSet = bucket_objectv_2::create(
///         "myThreatIntelSet",
///         BucketObjectv2Args::builder()
///             .acl("public-read")
///             .bucket("${bucket.id}")
///             .content("10.0.0.0/8\n")
///             .key("MyThreatIntelSet")
///             .build_struct(),
///     );
///     let myThreatIntelSetThreatIntelSet = threat_intel_set::create(
///         "myThreatIntelSetThreatIntelSet",
///         ThreatIntelSetArgs::builder()
///             .activate(true)
///             .detector_id("${primary.id}")
///             .format("TXT")
///             .location(
///                 "https://s3.amazonaws.com/${myThreatIntelSet.bucket}/${myThreatIntelSet.key}",
///             )
///             .name("MyThreatIntelSet")
///             .build_struct(),
///     );
///     let primary = detector::create(
///         "primary",
///         DetectorArgs::builder().enable(true).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import GuardDuty ThreatIntelSet using the primary GuardDuty detector ID and ThreatIntelSetID. For example:
///
/// ```sh
/// $ pulumi import aws:guardduty/threatIntelSet:ThreatIntelSet MyThreatIntelSet 00b00fd5aecc0ab60a708659477e9617:123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod threat_intel_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ThreatIntelSetArgs {
        /// Specifies whether GuardDuty is to start using the uploaded ThreatIntelSet.
        #[builder(into)]
        pub activate: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The detector ID of the GuardDuty.
        #[builder(into)]
        pub detector_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The format of the file that contains the ThreatIntelSet. Valid values: `TXT` | `STIX` | `OTX_CSV` | `ALIEN_VAULT` | `PROOF_POINT` | `FIRE_EYE`
        #[builder(into)]
        pub format: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The URI of the file that contains the ThreatIntelSet.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The friendly name to identify the ThreatIntelSet.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ThreatIntelSetResult {
        /// Specifies whether GuardDuty is to start using the uploaded ThreatIntelSet.
        pub activate: pulumi_gestalt_rust::Output<bool>,
        /// Amazon Resource Name (ARN) of the GuardDuty ThreatIntelSet.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The detector ID of the GuardDuty.
        pub detector_id: pulumi_gestalt_rust::Output<String>,
        /// The format of the file that contains the ThreatIntelSet. Valid values: `TXT` | `STIX` | `OTX_CSV` | `ALIEN_VAULT` | `PROOF_POINT` | `FIRE_EYE`
        pub format: pulumi_gestalt_rust::Output<String>,
        /// The URI of the file that contains the ThreatIntelSet.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The friendly name to identify the ThreatIntelSet.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ThreatIntelSetArgs,
    ) -> ThreatIntelSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let activate_binding_1 = args.activate.get_output(context);
        let activate_binding = activate_binding_1.get_inner();
        let detector_id_binding_1 = args.detector_id.get_output(context);
        let detector_id_binding = detector_id_binding_1.get_inner();
        let format_binding_1 = args.format.get_output(context);
        let format_binding = format_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:guardduty/threatIntelSet:ThreatIntelSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "activate".into(),
                    value: &activate_binding,
                },
                register_interface::ObjectField {
                    name: "detectorId".into(),
                    value: &detector_id_binding,
                },
                register_interface::ObjectField {
                    name: "format".into(),
                    value: &format_binding,
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
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ThreatIntelSetResult {
            activate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("activate"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            detector_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("detectorId"),
            ),
            format: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("format"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
