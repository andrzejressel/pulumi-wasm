#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_patch_baselines {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPatchBaselinesArgs {
        /// Only return baseline identities where `default_baseline` is `true`.
        #[builder(into, default)]
        pub default_baselines: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Key-value pairs used to filter the results. See `filter` below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ssm::GetPatchBaselinesFilter>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPatchBaselinesResult {
        /// List of baseline identities. See `baseline_identities` below.
        pub baseline_identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ssm::GetPatchBaselinesBaselineIdentity>,
        >,
        pub default_baselines: pulumi_gestalt_rust::Output<Option<bool>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ssm::GetPatchBaselinesFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetPatchBaselinesArgs,
    ) -> GetPatchBaselinesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let default_baselines_binding_1 = args.default_baselines.get_output(context);
        let default_baselines_binding = default_baselines_binding_1.get_inner();
        let filters_binding_1 = args.filters.get_output(context);
        let filters_binding = filters_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssm/getPatchBaselines:getPatchBaselines".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultBaselines".into(),
                    value: &default_baselines_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPatchBaselinesResult {
            baseline_identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("baselineIdentities"),
            ),
            default_baselines: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultBaselines"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
