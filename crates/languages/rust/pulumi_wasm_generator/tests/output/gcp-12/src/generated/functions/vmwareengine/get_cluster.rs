pub mod get_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// Name of the resource.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The resource name of the private cloud that this cluster belongs.
        #[builder(into)]
        pub parent: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        pub autoscaling_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetClusterAutoscalingSetting>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub management: pulumi_wasm_rust::Output<bool>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub node_type_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetClusterNodeTypeConfig>,
        >,
        pub parent: pulumi_wasm_rust::Output<String>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub uid: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetClusterArgs,
    ) -> GetClusterResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:vmwareengine/getCluster:getCluster".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetClusterResult {
            autoscaling_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoscalingSettings"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            management: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("management"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            node_type_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nodeTypeConfigs"),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(o.extract_field("parent")),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            uid: pulumi_wasm_rust::__private::into_domain(o.extract_field("uid")),
        }
    }
}
