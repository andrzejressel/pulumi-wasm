/// Resource for managing an AWS SESv2 (Simple Email V2) Dedicated IP Pool.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dedicated_ip_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DedicatedIpPoolArgs {
        /// Name of the dedicated IP pool.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub pool_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// IP pool scaling mode. Valid values: `STANDARD`, `MANAGED`. If omitted, the AWS API will default to a standard pool.
        #[builder(into, default)]
        pub scaling_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the pool. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DedicatedIpPoolResult {
        /// ARN of the Dedicated IP Pool.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the dedicated IP pool.
        ///
        /// The following arguments are optional:
        pub pool_name: pulumi_gestalt_rust::Output<String>,
        /// IP pool scaling mode. Valid values: `STANDARD`, `MANAGED`. If omitted, the AWS API will default to a standard pool.
        pub scaling_mode: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the pool. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
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
        args: DedicatedIpPoolArgs,
    ) -> DedicatedIpPoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let pool_name_binding = args.pool_name.get_output(context);
        let scaling_mode_binding = args.scaling_mode.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sesv2/dedicatedIpPool:DedicatedIpPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "poolName".into(),
                    value: pool_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scalingMode".into(),
                    value: scaling_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DedicatedIpPoolResult {
            arn: o.get_field("arn"),
            pool_name: o.get_field("poolName"),
            scaling_mode: o.get_field("scalingMode"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
