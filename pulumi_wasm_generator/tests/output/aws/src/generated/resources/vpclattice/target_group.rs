/// Resource for managing an AWS VPC Lattice Target Group.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = target_group::create(
///         "example",
///         TargetGroupArgs::builder()
///             .config(
///                 TargetGroupConfig::builder()
///                     .port(443)
///                     .protocol("HTTPS")
///                     .vpcIdentifier("${exampleAwsVpc.id}")
///                     .build_struct(),
///             )
///             .name("example")
///             .type_("INSTANCE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Basic usage with Health check
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = target_group::create(
///         "example",
///         TargetGroupArgs::builder()
///             .config(
///                 TargetGroupConfig::builder()
///                     .healthCheck(
///                         TargetGroupConfigHealthCheck::builder()
///                             .enabled(true)
///                             .healthCheckIntervalSeconds(20)
///                             .healthCheckTimeoutSeconds(10)
///                             .healthyThresholdCount(7)
///                             .matcher(
///                                 TargetGroupConfigHealthCheckMatcher::builder()
///                                     .value("200-299")
///                                     .build_struct(),
///                             )
///                             .path("/instance")
///                             .port(80)
///                             .protocol("HTTP")
///                             .protocolVersion("HTTP1")
///                             .unhealthyThresholdCount(3)
///                             .build_struct(),
///                     )
///                     .ipAddressType("IPV4")
///                     .port(443)
///                     .protocol("HTTPS")
///                     .protocolVersion("HTTP1")
///                     .vpcIdentifier("${exampleAwsVpc.id}")
///                     .build_struct(),
///             )
///             .name("example")
///             .type_("IP")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### ALB
///
/// If the type is ALB, `health_check` block is not supported.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = target_group::create(
///         "example",
///         TargetGroupArgs::builder()
///             .config(
///                 TargetGroupConfig::builder()
///                     .port(443)
///                     .protocol("HTTPS")
///                     .protocolVersion("HTTP1")
///                     .vpcIdentifier("${exampleAwsVpc.id}")
///                     .build_struct(),
///             )
///             .name("example")
///             .type_("ALB")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Lambda
///
/// If the type is Lambda, `config` block is not supported.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = target_group::create(
///         "example",
///         TargetGroupArgs::builder().name("example").type_("LAMBDA").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Lattice Target Group using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:vpclattice/targetGroup:TargetGroup example tg-0c11d4dc16ed96bdb
/// ```
pub mod target_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetGroupArgs {
        /// The target group configuration.
        #[builder(into, default)]
        pub config: pulumi_wasm_rust::Output<
            Option<super::super::types::vpclattice::TargetGroupConfig>,
        >,
        /// The name of the target group. The name must be unique within the account. The valid characters are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last character, or immediately after another hyphen.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of target group. Valid Values are `IP` | `LAMBDA` | `INSTANCE` | `ALB`
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TargetGroupResult {
        /// ARN of the target group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The target group configuration.
        pub config: pulumi_wasm_rust::Output<
            Option<super::super::types::vpclattice::TargetGroupConfig>,
        >,
        /// The name of the target group. The name must be unique within the account. The valid characters are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last character, or immediately after another hyphen.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Status of the target group.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of target group. Valid Values are `IP` | `LAMBDA` | `INSTANCE` | `ALB`
        ///
        /// The following arguments are optional:
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TargetGroupArgs) -> TargetGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let config_binding = args.config.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:vpclattice/targetGroup:TargetGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "config".into(),
                    value: &config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "config".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TargetGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("config").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
