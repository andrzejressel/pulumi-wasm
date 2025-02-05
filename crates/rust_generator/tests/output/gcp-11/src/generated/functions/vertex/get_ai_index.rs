pub mod get_ai_index {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAiIndexArgs {
        /// The name of the index.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The region of the index.
        ///
        /// - - -
        #[builder(into)]
        pub region: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAiIndexResult {
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub deployed_indexes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::vertex::GetAiIndexDeployedIndex>,
        >,
        pub description: pulumi_wasm_rust::Output<String>,
        pub display_name: pulumi_wasm_rust::Output<String>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub index_stats: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::vertex::GetAiIndexIndexStat>,
        >,
        pub index_update_method: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub metadata_schema_uri: pulumi_wasm_rust::Output<String>,
        pub metadatas: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::vertex::GetAiIndexMetadata>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub region: pulumi_wasm_rust::Output<String>,
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAiIndexArgs,
    ) -> GetAiIndexResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:vertex/getAiIndex:getAiIndex".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAiIndexResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            deployed_indexes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deployedIndexes"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            index_stats: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("indexStats"),
            ),
            index_update_method: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("indexUpdateMethod"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            metadata_schema_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metadataSchemaUri"),
            ),
            metadatas: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metadatas"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
