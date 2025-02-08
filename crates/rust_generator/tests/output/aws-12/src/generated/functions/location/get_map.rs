#[allow(clippy::doc_lazy_continuation)]
pub mod get_map {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMapArgs {
        /// Name of the map resource.
        #[builder(into)]
        pub map_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the map.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetMapResult {
        /// List of configurations that specify the map tile style selected from a partner data provider.
        pub configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::location::GetMapConfiguration>,
        >,
        /// Timestamp for when the map resource was created in ISO 8601 format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Optional description for the map resource.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN for the map resource.
        pub map_arn: pulumi_gestalt_rust::Output<String>,
        pub map_name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the map.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Timestamp for when the map resource was last updated in ISO 8601 format.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetMapArgs,
    ) -> GetMapResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let map_name_binding = args.map_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:location/getMap:getMap".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "mapName".into(),
                    value: &map_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetMapResult {
            configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configurations"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            map_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mapArn"),
            ),
            map_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mapName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
