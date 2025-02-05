pub mod get_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImageArgs {
        /// ID of the image.
        #[builder(into)]
        pub image_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetImageResult {
        /// The description of the image.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub image_id: pulumi_wasm_rust::Output<String>,
        /// The name of the image.
        pub name: pulumi_wasm_rust::Output<String>,
        pub operating_system_type: pulumi_wasm_rust::Output<String>,
        /// Specifies whether the image is running on dedicated hardware. When Bring Your Own License (BYOL) is enabled, this value is set to DEDICATED. For more information, see [Bring Your Own Windows Desktop Images](https://docs.aws.amazon.com/workspaces/latest/adminguide/byol-windows-images.html).
        pub required_tenancy: pulumi_wasm_rust::Output<String>,
        /// The status of the image.
        pub state: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetImageArgs,
    ) -> GetImageResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let image_id_binding = args.image_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:workspaces/getImage:getImage".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "imageId".into(),
                    value: &image_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetImageResult {
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            image_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("imageId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            operating_system_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("operatingSystemType"),
            ),
            required_tenancy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requiredTenancy"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
        }
    }
}
