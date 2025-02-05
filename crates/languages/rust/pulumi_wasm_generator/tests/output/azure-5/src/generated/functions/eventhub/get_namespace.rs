pub mod get_namespace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNamespaceArgs {
        /// The name of the EventHub Namespace.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the EventHub Namespace exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNamespaceResult {
        /// Is Auto Inflate enabled for the EventHub Namespace?
        pub auto_inflate_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Capacity / Throughput Units for a `Standard` SKU namespace.
        pub capacity: pulumi_wasm_rust::Output<i32>,
        /// The ID of the EventHub Dedicated Cluster where this Namespace exists.
        pub dedicated_cluster_id: pulumi_wasm_rust::Output<String>,
        /// The primary connection string for the authorization
        /// rule `RootManageSharedAccessKey`.
        pub default_primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The alias of the primary connection string for the authorization
        /// rule `RootManageSharedAccessKey`.
        pub default_primary_connection_string_alias: pulumi_wasm_rust::Output<String>,
        /// The primary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_primary_key: pulumi_wasm_rust::Output<String>,
        /// The secondary connection string for the
        /// authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The alias of the secondary connection string for the
        /// authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_connection_string_alias: pulumi_wasm_rust::Output<String>,
        /// The secondary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_key: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub kafka_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Azure location where the EventHub Namespace exists
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the maximum number of throughput units when Auto Inflate is Enabled.
        pub maximum_throughput_units: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Defines which tier to use.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the EventHub Namespace.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetNamespaceArgs,
    ) -> GetNamespaceResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:eventhub/getNamespace:getNamespace".into(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetNamespaceResult {
            auto_inflate_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoInflateEnabled"),
            ),
            capacity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("capacity"),
            ),
            dedicated_cluster_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dedicatedClusterId"),
            ),
            default_primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultPrimaryConnectionString"),
            ),
            default_primary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultPrimaryConnectionStringAlias"),
            ),
            default_primary_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultPrimaryKey"),
            ),
            default_secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultSecondaryConnectionString"),
            ),
            default_secondary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultSecondaryConnectionStringAlias"),
            ),
            default_secondary_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultSecondaryKey"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            kafka_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kafkaEnabled"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            maximum_throughput_units: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maximumThroughputUnits"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(o.extract_field("sku")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
