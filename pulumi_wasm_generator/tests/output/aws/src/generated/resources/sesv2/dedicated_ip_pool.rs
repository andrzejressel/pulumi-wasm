/// Resource for managing an AWS SESv2 (Simple Email V2) Dedicated IP Pool.
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
///     let example = dedicated_ip_pool::create(
///         "example",
///         DedicatedIpPoolArgs::builder().pool_name("my-pool").build_struct(),
///     );
/// }
/// ```
///
/// ### Managed Pool
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = dedicated_ip_pool::create(
///         "example",
///         DedicatedIpPoolArgs::builder()
///             .pool_name("my-managed-pool")
///             .scaling_mode("MANAGED")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SESv2 (Simple Email V2) Dedicated IP Pool using the `pool_name`. For example:
///
/// ```sh
/// $ pulumi import aws:sesv2/dedicatedIpPool:DedicatedIpPool example my-pool
/// ```
pub mod dedicated_ip_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DedicatedIpPoolArgs {
        /// Name of the dedicated IP pool.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub pool_name: pulumi_wasm_rust::Output<String>,
        /// IP pool scaling mode. Valid values: `STANDARD`, `MANAGED`. If omitted, the AWS API will default to a standard pool.
        #[builder(into, default)]
        pub scaling_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the pool. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DedicatedIpPoolResult {
        /// ARN of the Dedicated IP Pool.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name of the dedicated IP pool.
        ///
        /// The following arguments are optional:
        pub pool_name: pulumi_wasm_rust::Output<String>,
        /// IP pool scaling mode. Valid values: `STANDARD`, `MANAGED`. If omitted, the AWS API will default to a standard pool.
        pub scaling_mode: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the pool. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DedicatedIpPoolArgs) -> DedicatedIpPoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let pool_name_binding = args.pool_name.get_inner();
        let scaling_mode_binding = args.scaling_mode.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sesv2/dedicatedIpPool:DedicatedIpPool".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "poolName".into(),
                    value: &pool_name_binding,
                },
                register_interface::ObjectField {
                    name: "scalingMode".into(),
                    value: &scaling_mode_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "poolName".into(),
                },
                register_interface::ResultField {
                    name: "scalingMode".into(),
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
        DedicatedIpPoolResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            pool_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("poolName").unwrap(),
            ),
            scaling_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scalingMode").unwrap(),
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