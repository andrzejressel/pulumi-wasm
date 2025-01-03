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
pub mod landing_zone {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LandingZoneArgs {
        /// The manifest JSON file is a text file that describes your AWS resources. For examples, review [Launch your landing zone](https://docs.aws.amazon.com/controltower/latest/userguide/lz-api-launch).
        #[builder(into)]
        pub manifest_json: pulumi_wasm_rust::Output<String>,
        /// Tags to apply to the landing zone. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The landing zone version.
        #[builder(into)]
        pub version: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LandingZoneResult {
        /// The ARN of the landing zone.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The drift status summary of the landing zone.
        pub drift_statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::controltower::LandingZoneDriftStatus>,
        >,
        /// The latest available version of the landing zone.
        pub latest_available_version: pulumi_wasm_rust::Output<String>,
        /// The manifest JSON file is a text file that describes your AWS resources. For examples, review [Launch your landing zone](https://docs.aws.amazon.com/controltower/latest/userguide/lz-api-launch).
        pub manifest_json: pulumi_wasm_rust::Output<String>,
        /// Tags to apply to the landing zone. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the landing zone, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The landing zone version.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LandingZoneArgs) -> LandingZoneResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let manifest_json_binding = args.manifest_json.get_inner();
        let tags_binding = args.tags.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:controltower/landingZone:LandingZone".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "driftStatuses".into(),
                },
                register_interface::ResultField {
                    name: "latestAvailableVersion".into(),
                },
                register_interface::ResultField {
                    name: "manifestJson".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LandingZoneResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            drift_statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("driftStatuses").unwrap(),
            ),
            latest_available_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latestAvailableVersion").unwrap(),
            ),
            manifest_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("manifestJson").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
