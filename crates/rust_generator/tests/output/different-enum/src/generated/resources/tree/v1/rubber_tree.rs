#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod rubber_tree {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RubberTreeArgs {
        #[builder(into, default)]
        pub container: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::Container>,
        >,
        #[builder(into)]
        pub diameter: pulumi_gestalt_rust::InputOrOutput<
            super::super::super::types::tree::v1::Diameter,
        >,
        #[builder(into, default)]
        pub farm: pulumi_gestalt_rust::InputOrOutput<
            Option<
                pulumi_gestalt_rust::OneOf2<
                    super::super::super::types::tree::v1::Farm,
                    String,
                >,
            >,
        >,
        #[builder(into, default)]
        pub size: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::tree::v1::TreeSize>,
        >,
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<
            super::super::super::types::tree::v1::RubberTreeVariety,
        >,
    }
    #[allow(dead_code)]
    pub struct RubberTreeResult {
        pub container: pulumi_gestalt_rust::Output<
            Option<super::super::super::types::Container>,
        >,
        pub diameter: pulumi_gestalt_rust::Output<
            super::super::super::types::tree::v1::Diameter,
        >,
        pub farm: pulumi_gestalt_rust::Output<
            Option<
                pulumi_gestalt_rust::OneOf2<
                    super::super::super::types::tree::v1::Farm,
                    String,
                >,
            >,
        >,
        pub size: pulumi_gestalt_rust::Output<
            Option<super::super::super::types::tree::v1::TreeSize>,
        >,
        pub type_: pulumi_gestalt_rust::Output<
            super::super::super::types::tree::v1::RubberTreeVariety,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RubberTreeArgs,
    ) -> RubberTreeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let container_binding = args.container.get_output(context);
        let diameter_binding = args.diameter.get_output(context);
        let farm_binding = args.farm.get_output(context);
        let size_binding = args.size.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "plant:tree/v1:RubberTree".into(),
            name: name.to_string(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "container".into(),
                    value: &container_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diameter".into(),
                    value: &diameter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "farm".into(),
                    value: &farm_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "size".into(),
                    value: &size_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RubberTreeResult {
            container: o.get_field("container"),
            diameter: o.get_field("diameter"),
            farm: o.get_field("farm"),
            size: o.get_field("size"),
            type_: o.get_field("type"),
        }
    }
}
