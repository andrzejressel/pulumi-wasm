/// Manages the registration of a Resource Provider - which allows access to the API's supported by this Resource Provider.
///
/// > The Azure Provider will automatically register all of the Resource Providers which it supports on launch (unless opted-out using the `skip_provider_registration` field within the provider block).
///
/// !> **Note:** The errors returned from the Azure API when a Resource Provider is unregistered are unclear (example `API version '2019-01-01' was not found for 'Microsoft.Foo'`) - please ensure that all of the necessary Resource Providers you're using are registered - if in doubt **we strongly recommend letting the provider register these for you**.
///
/// > **Note:** Adding or Removing a Preview Feature will re-register the Resource Provider.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_provider_registration::create(
///         "example",
///         ResourceProviderRegistrationArgs::builder()
///             .name("Microsoft.PolicyInsights")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### Registering A Preview Feature)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_provider_registration::create(
///         "example",
///         ResourceProviderRegistrationArgs::builder()
///             .features(
///                 vec![
///                     ResourceProviderRegistrationFeature::builder()
///                     .name("AKS-DataPlaneAutoApprove").registered(true).build_struct(),
///                 ],
///             )
///             .name("Microsoft.ContainerService")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Resource Provider Registrations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:core/resourceProviderRegistration:ResourceProviderRegistration example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.PolicyInsights
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_provider_registration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceProviderRegistrationArgs {
        #[builder(into, default)]
        pub features: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::core::ResourceProviderRegistrationFeature>>,
        >,
        /// The namespace of the Resource Provider which should be registered. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ResourceProviderRegistrationResult {
        pub features: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::core::ResourceProviderRegistrationFeature>>,
        >,
        /// The namespace of the Resource Provider which should be registered. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceProviderRegistrationArgs,
    ) -> ResourceProviderRegistrationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let features_binding = args.features.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:core/resourceProviderRegistration:ResourceProviderRegistration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "features".into(),
                    value: &features_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceProviderRegistrationResult {
            features: o.get_field("features"),
            name: o.get_field("name"),
        }
    }
}
