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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ThreatIntelSetArgs,
    ) -> ThreatIntelSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let activate_binding = args.activate.get_output(context);
        let detector_id_binding = args.detector_id.get_output(context);
        let format_binding = args.format.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:guardduty/threatIntelSet:ThreatIntelSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "activate".into(),
                    value: activate_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "detectorId".into(),
                    value: detector_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "format".into(),
                    value: format_binding.get_id(),
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
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ThreatIntelSetResult {
            activate: o.get_field("activate"),
            arn: o.get_field("arn"),
            detector_id: o.get_field("detectorId"),
            format: o.get_field("format"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
