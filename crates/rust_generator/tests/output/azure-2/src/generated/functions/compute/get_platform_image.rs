#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_platform_image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPlatformImageArgs {
        /// Specifies the Location to pull information about this Platform Image from.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Offer associated with the Platform Image.
        #[builder(into)]
        pub offer: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Publisher associated with the Platform Image.
        #[builder(into)]
        pub publisher: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the SKU of the Platform Image.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The version of the Platform Image.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPlatformImageResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub offer: pulumi_gestalt_rust::Output<String>,
        pub publisher: pulumi_gestalt_rust::Output<String>,
        pub sku: pulumi_gestalt_rust::Output<String>,
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPlatformImageArgs,
    ) -> GetPlatformImageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let offer_binding = args.offer.get_output(context);
        let publisher_binding = args.publisher.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:compute/getPlatformImage:getPlatformImage".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "offer".into(),
                    value: offer_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publisher".into(),
                    value: publisher_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: sku_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPlatformImageResult {
            id: o.get_field("id"),
            location: o.get_field("location"),
            offer: o.get_field("offer"),
            publisher: o.get_field("publisher"),
            sku: o.get_field("sku"),
            version: o.get_field("version"),
        }
    }
}
