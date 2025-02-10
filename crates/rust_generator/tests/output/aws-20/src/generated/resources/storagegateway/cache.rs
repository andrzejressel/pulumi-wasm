/// Manages an AWS Storage Gateway cache.
///
/// > **NOTE:** The Storage Gateway API provides no method to remove a cache disk. Destroying this resource does not perform any Storage Gateway actions.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cache::create(
///         "example",
///         CacheArgs::builder()
///             .disk_id("${exampleAwsStoragegatewayLocalDisk.id}")
///             .gateway_arn("${exampleAwsStoragegatewayGateway.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_storagegateway_cache` using the gateway Amazon Resource Name (ARN) and local disk identifier separated with a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:storagegateway/cache:Cache example arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678:pci-0000:03:00.0-scsi-0:0:0:0
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cache {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CacheArgs {
        /// Local disk identifier. For example, `pci-0000:03:00.0-scsi-0:0:0:0`.
        #[builder(into)]
        pub disk_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the gateway.
        #[builder(into)]
        pub gateway_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CacheResult {
        /// Local disk identifier. For example, `pci-0000:03:00.0-scsi-0:0:0:0`.
        pub disk_id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the gateway.
        pub gateway_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CacheArgs,
    ) -> CacheResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let disk_id_binding = args.disk_id.get_output(context);
        let gateway_arn_binding = args.gateway_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:storagegateway/cache:Cache".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskId".into(),
                    value: disk_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayArn".into(),
                    value: gateway_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CacheResult {
            disk_id: o.get_field("diskId"),
            gateway_arn: o.get_field("gatewayArn"),
        }
    }
}
