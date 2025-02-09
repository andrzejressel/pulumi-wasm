#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_access_points {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccessPointsArgs {
        /// EFS File System identifier.
        #[builder(into)]
        pub file_system_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccessPointsResult {
        /// Set of Amazon Resource Names (ARNs).
        pub arns: pulumi_gestalt_rust::Output<Vec<String>>,
        pub file_system_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Set of identifiers.
        pub ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAccessPointsArgs,
    ) -> GetAccessPointsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let file_system_id_binding_1 = args.file_system_id.get_output(context);
        let file_system_id_binding = file_system_id_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:efs/getAccessPoints:getAccessPoints".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "fileSystemId".into(),
                    value: &file_system_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAccessPointsResult {
            arns: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arns")),
            file_system_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fileSystemId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ids: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ids")),
        }
    }
}
