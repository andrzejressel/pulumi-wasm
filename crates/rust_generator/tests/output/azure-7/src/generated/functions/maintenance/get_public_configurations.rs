#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_public_configurations {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPublicConfigurationsArgs {
        /// The Azure location to filter the list of Public Maintenance Configurations against.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The recurring window to filter the list of Public Maintenance Configurations against. Possible values are `Monday-Thursday` and `Friday-Sunday`
        #[builder(into, default)]
        pub recur_every: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The scope to filter the list of Public Maintenance Configurations against. Possible values are `Extension`, `Host`, `InGuestPatch`, `OSImage`, `SQLDB` and `SQLManagedInstance`.
        #[builder(into, default)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPublicConfigurationsResult {
        /// A `configs` block as defined below.
        pub configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::maintenance::GetPublicConfigurationsConfig>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure location of the Public Maintenance Configuration.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The rate at which a maintenance window is expected to recur.
        pub recur_every: pulumi_gestalt_rust::Output<Option<String>>,
        pub scope: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPublicConfigurationsArgs,
    ) -> GetPublicConfigurationsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let recur_every_binding = args.recur_every.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:maintenance/getPublicConfigurations:getPublicConfigurations"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recurEvery".into(),
                    value: &recur_every_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: &scope_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPublicConfigurationsResult {
            configs: o.get_field("configs"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            recur_every: o.get_field("recurEvery"),
            scope: o.get_field("scope"),
        }
    }
}
