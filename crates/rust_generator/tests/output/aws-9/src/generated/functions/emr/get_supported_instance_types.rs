#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_supported_instance_types {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSupportedInstanceTypesArgs {
        /// Amazon EMR release label. For more information about Amazon EMR releases and their included application versions and features, see the [Amazon EMR Release Guide](https://docs.aws.amazon.com/emr/latest/ReleaseGuide/emr-release-components.html).
        #[builder(into)]
        pub release_label: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of supported instance types. See `supported_instance_types` below.
        #[builder(into, default)]
        pub supported_instance_types: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::emr::GetSupportedInstanceTypesSupportedInstanceType,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSupportedInstanceTypesResult {
        pub id: pulumi_gestalt_rust::Output<String>,
        pub release_label: pulumi_gestalt_rust::Output<String>,
        /// List of supported instance types. See `supported_instance_types` below.
        pub supported_instance_types: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::emr::GetSupportedInstanceTypesSupportedInstanceType,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSupportedInstanceTypesArgs,
    ) -> GetSupportedInstanceTypesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let release_label_binding = args.release_label.get_output(context);
        let supported_instance_types_binding = args
            .supported_instance_types
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:emr/getSupportedInstanceTypes:getSupportedInstanceTypes".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "releaseLabel".into(),
                    value: release_label_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "supportedInstanceTypes".into(),
                    value: supported_instance_types_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSupportedInstanceTypesResult {
            id: o.get_field("id"),
            release_label: o.get_field("releaseLabel"),
            supported_instance_types: o.get_field("supportedInstanceTypes"),
        }
    }
}
