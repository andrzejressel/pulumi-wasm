/// Provides a resource to manage a GuardDuty IPSet.
///
/// > **Note:** Currently in GuardDuty, users from member accounts cannot upload and further manage IPSets. IPSets that are uploaded by the primary account are imposed on GuardDuty functionality in its member accounts. See the [GuardDuty API Documentation](https://docs.aws.amazon.com/guardduty/latest/ug/create-ip-set.html)
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod ip_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IPSetArgs {
        /// Specifies whether GuardDuty is to start using the uploaded IPSet.
        #[builder(into)]
        pub activate: pulumi_wasm_rust::Output<bool>,
        /// The detector ID of the GuardDuty.
        #[builder(into)]
        pub detector_id: pulumi_wasm_rust::Output<String>,
        /// The format of the file that contains the IPSet. Valid values: `TXT` | `STIX` | `OTX_CSV` | `ALIEN_VAULT` | `PROOF_POINT` | `FIRE_EYE`
        #[builder(into)]
        pub format: pulumi_wasm_rust::Output<String>,
        /// The URI of the file that contains the IPSet.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The friendly name to identify the IPSet.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct IPSetResult {
        /// Specifies whether GuardDuty is to start using the uploaded IPSet.
        pub activate: pulumi_wasm_rust::Output<bool>,
        /// Amazon Resource Name (ARN) of the GuardDuty IPSet.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The detector ID of the GuardDuty.
        pub detector_id: pulumi_wasm_rust::Output<String>,
        /// The format of the file that contains the IPSet. Valid values: `TXT` | `STIX` | `OTX_CSV` | `ALIEN_VAULT` | `PROOF_POINT` | `FIRE_EYE`
        pub format: pulumi_wasm_rust::Output<String>,
        /// The URI of the file that contains the IPSet.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The friendly name to identify the IPSet.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: IPSetArgs) -> IPSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let activate_binding = args.activate.get_inner();
        let detector_id_binding = args.detector_id.get_inner();
        let format_binding = args.format.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:guardduty/iPSet:IPSet".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "activate".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "detectorId".into(),
                },
                register_interface::ResultField {
                    name: "format".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IPSetResult {
            activate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activate").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            detector_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("detectorId").unwrap(),
            ),
            format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("format").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}