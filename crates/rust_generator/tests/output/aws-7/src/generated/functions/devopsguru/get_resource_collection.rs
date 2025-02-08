#[allow(clippy::doc_lazy_continuation)]
pub mod get_resource_collection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResourceCollectionArgs {
        /// A collection of AWS CloudFormation stacks. See `cloudformation` below for additional details.
        #[builder(into, default)]
        pub cloudformations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::devopsguru::GetResourceCollectionCloudformation,
                >,
            >,
        >,
        /// AWS tags used to filter the resources in the resource collection. See `tags` below for additional details.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::devopsguru::GetResourceCollectionTag>>,
        >,
        /// Type of AWS resource collection to create. Valid values are `AWS_CLOUD_FORMATION`, `AWS_SERVICE`, and `AWS_TAGS`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResourceCollectionResult {
        /// A collection of AWS CloudFormation stacks. See `cloudformation` below for additional details.
        pub cloudformations: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::devopsguru::GetResourceCollectionCloudformation,
                >,
            >,
        >,
        /// Type of AWS resource collection to create (same value as `type`).
        pub id: pulumi_gestalt_rust::Output<String>,
        /// AWS tags used to filter the resources in the resource collection. See `tags` below for additional details.
        pub tags: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::devopsguru::GetResourceCollectionTag>>,
        >,
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetResourceCollectionArgs,
    ) -> GetResourceCollectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cloudformations_binding = args
            .cloudformations
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:devopsguru/getResourceCollection:getResourceCollection".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudformations".into(),
                    value: &cloudformations_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetResourceCollectionResult {
            cloudformations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cloudformations"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
