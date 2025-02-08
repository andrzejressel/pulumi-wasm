/// Provides a resource to manage a GuardDuty IPSet.
///
/// > **Note:** Currently in GuardDuty, users from member accounts cannot upload and further manage IPSets. IPSets that are uploaded by the primary account are imposed on GuardDuty functionality in its member accounts. See the [GuardDuty API Documentation](https://docs.aws.amazon.com/guardduty/latest/ug/create-ip-set.html)
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
///     let example = ip_set::create(
///         "example",
///         IpSetArgs::builder()
///             .activate(true)
///             .detector_id("${primary.id}")
///             .format("TXT")
///             .location("https://s3.amazonaws.com/${myIPSet.bucket}/${myIPSet.key}")
///             .name("MyIPSet")
///             .build_struct(),
///     );
///     let myIPSet = bucket_objectv_2::create(
///         "myIPSet",
///         BucketObjectv2Args::builder()
///             .bucket("${bucket.id}")
///             .content("10.0.0.0/8\n")
///             .key("MyIPSet")
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
/// Using `pulumi import`, import GuardDuty IPSet using the primary GuardDuty detector ID and IPSet ID. For example:
///
/// ```sh
/// $ pulumi import aws:guardduty/iPSet:IPSet MyIPSet 00b00fd5aecc0ab60a708659477e9617:123456789012
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod ip_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IPSetArgs {
        /// Specifies whether GuardDuty is to start using the uploaded IPSet.
        #[builder(into)]
        pub activate: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The detector ID of the GuardDuty.
        #[builder(into)]
        pub detector_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The format of the file that contains the IPSet. Valid values: `TXT` | `STIX` | `OTX_CSV` | `ALIEN_VAULT` | `PROOF_POINT` | `FIRE_EYE`
        #[builder(into)]
        pub format: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The URI of the file that contains the IPSet.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The friendly name to identify the IPSet.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct IPSetResult {
        /// Specifies whether GuardDuty is to start using the uploaded IPSet.
        pub activate: pulumi_gestalt_rust::Output<bool>,
        /// Amazon Resource Name (ARN) of the GuardDuty IPSet.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The detector ID of the GuardDuty.
        pub detector_id: pulumi_gestalt_rust::Output<String>,
        /// The format of the file that contains the IPSet. Valid values: `TXT` | `STIX` | `OTX_CSV` | `ALIEN_VAULT` | `PROOF_POINT` | `FIRE_EYE`
        pub format: pulumi_gestalt_rust::Output<String>,
        /// The URI of the file that contains the IPSet.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The friendly name to identify the IPSet.
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
        args: IPSetArgs,
    ) -> IPSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let activate_binding = args.activate.get_output(context).get_inner();
        let detector_id_binding = args.detector_id.get_output(context).get_inner();
        let format_binding = args.format.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:guardduty/iPSet:IPSet".into(),
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
        IPSetResult {
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
