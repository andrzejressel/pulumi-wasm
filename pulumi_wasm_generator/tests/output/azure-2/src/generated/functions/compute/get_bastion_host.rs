pub mod get_bastion_host {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBastionHostArgs {
        /// The name of the Bastion Host.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Bastion Host exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetBastionHostResult {
        /// Is Copy/Paste feature enabled for the Bastion Host.
        pub copy_paste_enabled: pulumi_wasm_rust::Output<bool>,
        /// The FQDN for the Bastion Host.
        pub dns_name: pulumi_wasm_rust::Output<String>,
        /// Is File Copy feature enabled for the Bastion Host.
        pub file_copy_enabled: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A `ip_configuration` block as defined below.
        pub ip_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetBastionHostIpConfiguration>,
        >,
        /// Is IP Connect feature enabled for the Bastion Host.
        pub ip_connect_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Azure Region where the Bastion Host exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the IP configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The number of scale units provisioned for the Bastion Host.
        pub scale_units: pulumi_wasm_rust::Output<i32>,
        /// Is Session Recording feature enabled for the Bastion Host.
        pub session_recording_enabled: pulumi_wasm_rust::Output<bool>,
        /// Is Shareable Link feature enabled for the Bastion Host.
        pub shareable_link_enabled: pulumi_wasm_rust::Output<bool>,
        /// The SKU of the Bastion Host.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the Bastion Host.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Is Tunneling feature enabled for the Bastion Host.
        pub tunneling_enabled: pulumi_wasm_rust::Output<bool>,
        /// A list of Availability Zones in which this Bastion Host is located.
        pub zones: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetBastionHostArgs) -> GetBastionHostResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:compute/getBastionHost:getBastionHost".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "copyPasteEnabled".into(),
                },
                register_interface::ResultField {
                    name: "dnsName".into(),
                },
                register_interface::ResultField {
                    name: "fileCopyEnabled".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "ipConnectEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "scaleUnits".into(),
                },
                register_interface::ResultField {
                    name: "sessionRecordingEnabled".into(),
                },
                register_interface::ResultField {
                    name: "shareableLinkEnabled".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tunnelingEnabled".into(),
                },
                register_interface::ResultField {
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBastionHostResult {
            copy_paste_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("copyPasteEnabled").unwrap(),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsName").unwrap(),
            ),
            file_copy_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileCopyEnabled").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipConfigurations").unwrap(),
            ),
            ip_connect_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipConnectEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            scale_units: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scaleUnits").unwrap(),
            ),
            session_recording_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sessionRecordingEnabled").unwrap(),
            ),
            shareable_link_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shareableLinkEnabled").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tunneling_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnelingEnabled").unwrap(),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}
