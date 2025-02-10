#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_frontdoor_secret {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFrontdoorSecretArgs {
        /// Specifies the name of the Front Door Secret.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Front Door Profile within which the Front Door Secret exists.
        #[builder(into)]
        pub profile_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Front Door Profile exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFrontdoorSecretResult {
        /// Specifies the ID of the Front Door Profile within which this Front Door Secret exists.
        pub cdn_frontdoor_profile_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub profile_name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `secret` block as defined below.
        pub secrets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cdn::GetFrontdoorSecretSecret>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFrontdoorSecretArgs,
    ) -> GetFrontdoorSecretResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let profile_name_binding = args.profile_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:cdn/getFrontdoorSecret:getFrontdoorSecret".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "profileName".into(),
                    value: profile_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFrontdoorSecretResult {
            cdn_frontdoor_profile_id: o.get_field("cdnFrontdoorProfileId"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            profile_name: o.get_field("profileName"),
            resource_group_name: o.get_field("resourceGroupName"),
            secrets: o.get_field("secrets"),
        }
    }
}
