#[allow(clippy::doc_lazy_continuation)]
pub mod get_api {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApiArgs {
        /// The name of the API Management Service in which the API Management API exists.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the API Management API.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group in which the API Management Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Revision of the API Management API.
        #[builder(into)]
        pub revision: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetApiResult {
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// A description of the API Management API, which may include HTML formatting tags.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The display name of the API.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Is this the current API Revision?
        pub is_current: pulumi_gestalt_rust::Output<bool>,
        /// Is this API Revision online/accessible via the Gateway?
        pub is_online: pulumi_gestalt_rust::Output<bool>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Path for this API Management API.
        pub path: pulumi_gestalt_rust::Output<String>,
        /// A list of protocols the operations in this API can be invoked.
        pub protocols: pulumi_gestalt_rust::Output<Vec<String>>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub revision: pulumi_gestalt_rust::Output<String>,
        /// Absolute URL of the backend service implementing this API.
        pub service_url: pulumi_gestalt_rust::Output<String>,
        /// Should this API expose a SOAP frontend, rather than a HTTP frontend?
        pub soap_pass_through: pulumi_gestalt_rust::Output<bool>,
        /// A `subscription_key_parameter_names` block as documented below.
        pub subscription_key_parameter_names: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::apimanagement::GetApiSubscriptionKeyParameterName,
            >,
        >,
        /// Should this API require a subscription key?
        pub subscription_required: pulumi_gestalt_rust::Output<bool>,
        /// The Version number of this API, if this API is versioned.
        pub version: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Version Set which this API is associated with.
        pub version_set_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetApiArgs,
    ) -> GetApiResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args
            .api_management_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let revision_binding = args.revision.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:apimanagement/getApi:getApi".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "revision".into(),
                    value: &revision_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetApiResult {
            api_management_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiManagementName"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            is_current: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isCurrent"),
            ),
            is_online: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isOnline"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            path: pulumi_gestalt_rust::__private::into_domain(o.extract_field("path")),
            protocols: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protocols"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            revision: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("revision"),
            ),
            service_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceUrl"),
            ),
            soap_pass_through: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("soapPassThrough"),
            ),
            subscription_key_parameter_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subscriptionKeyParameterNames"),
            ),
            subscription_required: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subscriptionRequired"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            version_set_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionSetId"),
            ),
        }
    }
}
