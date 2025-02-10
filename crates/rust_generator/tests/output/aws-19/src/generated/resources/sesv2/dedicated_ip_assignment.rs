/// Resource for managing an AWS SESv2 (Simple Email V2) Dedicated IP Assignment.
///
/// This resource is used with "Standard" dedicated IP addresses. This includes addresses [requested and relinquished manually](https://docs.aws.amazon.com/ses/latest/dg/dedicated-ip-case.html) via an AWS support case, or [Bring Your Own IP](https://docs.aws.amazon.com/ses/latest/dg/dedicated-ip-byo.html) addresses. Once no longer assigned, this resource returns the IP to the [`ses-default-dedicated-pool`](https://docs.aws.amazon.com/ses/latest/dg/managing-ip-pools.html), managed by AWS.
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
///     let example = dedicated_ip_assignment::create(
///         "example",
///         DedicatedIpAssignmentArgs::builder()
///             .destination_pool_name("my-pool")
///             .ip("0.0.0.0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SESv2 (Simple Email V2) Dedicated IP Assignment using the `id`, which is a comma-separated string made up of `ip` and `destination_pool_name`. For example:
///
/// ```sh
/// $ pulumi import aws:sesv2/dedicatedIpAssignment:DedicatedIpAssignment example "0.0.0.0,my-pool"
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dedicated_ip_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DedicatedIpAssignmentArgs {
        /// Dedicated IP address.
        #[builder(into)]
        pub destination_pool_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Dedicated IP address.
        #[builder(into)]
        pub ip: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DedicatedIpAssignmentResult {
        /// Dedicated IP address.
        pub destination_pool_name: pulumi_gestalt_rust::Output<String>,
        /// Dedicated IP address.
        pub ip: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DedicatedIpAssignmentArgs,
    ) -> DedicatedIpAssignmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destination_pool_name_binding = args
            .destination_pool_name
            .get_output(context);
        let ip_binding = args.ip.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sesv2/dedicatedIpAssignment:DedicatedIpAssignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationPoolName".into(),
                    value: destination_pool_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ip".into(),
                    value: ip_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DedicatedIpAssignmentResult {
            destination_pool_name: o.get_field("destinationPoolName"),
            ip: o.get_field("ip"),
        }
    }
}
