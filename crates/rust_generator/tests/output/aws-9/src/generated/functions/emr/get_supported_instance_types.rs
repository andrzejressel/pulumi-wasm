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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSupportedInstanceTypesArgs,
    ) -> GetSupportedInstanceTypesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let release_label_binding_1 = args.release_label.get_output(context);
        let release_label_binding = release_label_binding_1.get_inner();
        let supported_instance_types_binding_1 = args
            .supported_instance_types
            .get_output(context);
        let supported_instance_types_binding = supported_instance_types_binding_1
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:emr/getSupportedInstanceTypes:getSupportedInstanceTypes".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "releaseLabel".into(),
                    value: &release_label_binding,
                },
                register_interface::ObjectField {
                    name: "supportedInstanceTypes".into(),
                    value: &supported_instance_types_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSupportedInstanceTypesResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            release_label: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("releaseLabel"),
            ),
            supported_instance_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportedInstanceTypes"),
            ),
        }
    }
}
