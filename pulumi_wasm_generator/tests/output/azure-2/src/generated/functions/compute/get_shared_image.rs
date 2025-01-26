pub mod get_shared_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSharedImageArgs {
        /// The name of the Shared Image Gallery in which the Shared Image exists.
        #[builder(into)]
        pub gallery_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Shared Image.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Shared Image Gallery exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSharedImageResult {
        /// Specifies if the Shared Image supports Accelerated Network.
        pub accelerated_network_support_enabled: pulumi_wasm_rust::Output<bool>,
        pub architecture: pulumi_wasm_rust::Output<String>,
        /// Specifies if Confidential Virtual Machines enabled. It will enable all the features of trusted, with higher confidentiality features for isolate machines or encrypted data. Available for Gen2 machines.
        pub confidential_vm_enabled: pulumi_wasm_rust::Output<bool>,
        /// Specifies if supports creation of both Confidential virtual machines and Gen2 virtual machines with standard security from a compatible Gen2 OS disk VHD or Gen2 Managed image.
        pub confidential_vm_supported: pulumi_wasm_rust::Output<bool>,
        /// The description of this Shared Image.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The End User Licence Agreement for the Shared Image.
        pub eula: pulumi_wasm_rust::Output<String>,
        pub gallery_name: pulumi_wasm_rust::Output<String>,
        /// Specifies if the Shared Image supports hibernation.
        pub hibernation_enabled: pulumi_wasm_rust::Output<bool>,
        /// The generation of HyperV that the Virtual Machine used to create the Shared Image is based on.
        pub hyper_v_generation: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An `identifier` block as defined below.
        pub identifiers: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetSharedImageIdentifier>,
        >,
        /// The supported Azure location where the Shared Image Gallery exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// (Required) The Purchase Plan Name for this Shared Image.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The type of Operating System present in this Shared Image.
        pub os_type: pulumi_wasm_rust::Output<String>,
        /// The URI containing the Privacy Statement for this Shared Image.
        pub privacy_statement_uri: pulumi_wasm_rust::Output<String>,
        /// (Optional) A `purchase_plan` block as defined below.
        pub purchase_plans: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetSharedImagePurchasePlan>,
        >,
        /// The URI containing the Release Notes for this Shared Image.
        pub release_note_uri: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies that the Operating System used inside this Image has not been Generalized (for example, `sysprep` on Windows has not been run).
        pub specialized: pulumi_wasm_rust::Output<bool>,
        /// A mapping of tags assigned to the Shared Image.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Specifies if Trusted Launch has to be enabled for the Virtual Machine created from the Shared Image.
        pub trusted_launch_enabled: pulumi_wasm_rust::Output<bool>,
        /// Specifies if supports creation of both Trusted Launch virtual machines and Gen2 virtual machines with standard security created from the Shared Image.
        pub trusted_launch_supported: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSharedImageArgs,
    ) -> GetSharedImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let gallery_name_binding = args.gallery_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:compute/getSharedImage:getSharedImage".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "galleryName".into(),
                    value: &gallery_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSharedImageResult {
            accelerated_network_support_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("acceleratedNetworkSupportEnabled"),
            ),
            architecture: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("architecture"),
            ),
            confidential_vm_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("confidentialVmEnabled"),
            ),
            confidential_vm_supported: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("confidentialVmSupported"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            eula: pulumi_wasm_rust::__private::into_domain(o.extract_field("eula")),
            gallery_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("galleryName"),
            ),
            hibernation_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hibernationEnabled"),
            ),
            hyper_v_generation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hyperVGeneration"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identifiers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identifiers"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            os_type: pulumi_wasm_rust::__private::into_domain(o.extract_field("osType")),
            privacy_statement_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privacyStatementUri"),
            ),
            purchase_plans: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("purchasePlans"),
            ),
            release_note_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("releaseNoteUri"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            specialized: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("specialized"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            trusted_launch_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("trustedLaunchEnabled"),
            ),
            trusted_launch_supported: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("trustedLaunchSupported"),
            ),
        }
    }
}
