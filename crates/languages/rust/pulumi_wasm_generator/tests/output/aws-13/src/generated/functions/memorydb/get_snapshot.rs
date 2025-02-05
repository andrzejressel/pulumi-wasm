pub mod get_snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSnapshotArgs {
        /// Name of the snapshot.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Map of tags assigned to the snapshot.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSnapshotResult {
        /// ARN of the snapshot.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The configuration of the cluster from which the snapshot was taken.
        pub cluster_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::memorydb::GetSnapshotClusterConfiguration>,
        >,
        /// Name of the MemoryDB cluster that this snapshot was taken from.
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN of the KMS key used to encrypt the snapshot at rest.
        pub kms_key_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the cluster.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether the snapshot is from an automatic backup (`automated`) or was created manually (`manual`).
        pub source: pulumi_wasm_rust::Output<String>,
        /// Map of tags assigned to the snapshot.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSnapshotArgs,
    ) -> GetSnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:memorydb/getSnapshot:getSnapshot".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSnapshotResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            cluster_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterConfigurations"),
            ),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterName"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsKeyArn"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            source: pulumi_wasm_rust::__private::into_domain(o.extract_field("source")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
