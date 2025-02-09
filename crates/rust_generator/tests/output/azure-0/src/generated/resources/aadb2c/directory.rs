/// Manages an AAD B2C Directory.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = directory::create(
///         "example",
///         DirectoryArgs::builder()
///             .country_code("US")
///             .data_residency_location("United States")
///             .display_name("example-b2c-tenant")
///             .domain_name("exampleb2ctenant.onmicrosoft.com")
///             .resource_group_name("example-rg")
///             .sku_name("PremiumP1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// AAD B2C Directories can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:aadb2c/directory:Directory example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/example-resource-group/providers/Microsoft.AzureActiveDirectory/b2cDirectories/directory-name
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod directory {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DirectoryArgs {
        /// Country code of the B2C tenant. The `country_code` should be valid for the specified `data_residency_location`. See [official docs](https://aka.ms/B2CDataResidency) for valid country codes. Required when creating a new resource. Changing this forces a new AAD B2C Directory to be created.
        #[builder(into, default)]
        pub country_code: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Location in which the B2C tenant is hosted and data resides. The `data_residency_location` should be valid for the specified `country_code`. See [official docs](https://aka.ms/B2CDataResidenc) for more information. Changing this forces a new AAD B2C Directory to be created. Possible values are `Asia Pacific`, `Australia`, `Europe`, `Global` and `United States`.
        #[builder(into)]
        pub data_residency_location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The initial display name of the B2C tenant. Required when creating a new resource. Changing this forces a new AAD B2C Directory to be created.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Domain name of the B2C tenant, including the `.onmicrosoft.com` suffix. Changing this forces a new AAD B2C Directory to be created.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the AAD B2C Directory should exist. Changing this forces a new AAD B2C Directory to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Billing SKU for the B2C tenant. Must be one of: `PremiumP1` or `PremiumP2` (`Standard` is not supported). See [official docs](https://aka.ms/b2cBilling) for more information.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the AAD B2C Directory.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DirectoryResult {
        /// The type of billing for the AAD B2C tenant. Possible values include: `MAU` or `Auths`.
        pub billing_type: pulumi_gestalt_rust::Output<String>,
        /// Country code of the B2C tenant. The `country_code` should be valid for the specified `data_residency_location`. See [official docs](https://aka.ms/B2CDataResidency) for valid country codes. Required when creating a new resource. Changing this forces a new AAD B2C Directory to be created.
        pub country_code: pulumi_gestalt_rust::Output<String>,
        /// Location in which the B2C tenant is hosted and data resides. The `data_residency_location` should be valid for the specified `country_code`. See [official docs](https://aka.ms/B2CDataResidenc) for more information. Changing this forces a new AAD B2C Directory to be created. Possible values are `Asia Pacific`, `Australia`, `Europe`, `Global` and `United States`.
        pub data_residency_location: pulumi_gestalt_rust::Output<String>,
        /// The initial display name of the B2C tenant. Required when creating a new resource. Changing this forces a new AAD B2C Directory to be created.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Domain name of the B2C tenant, including the `.onmicrosoft.com` suffix. Changing this forces a new AAD B2C Directory to be created.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// The date from which the billing type took effect. May not be populated until after the first billing cycle.
        pub effective_start_date: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the AAD B2C Directory should exist. Changing this forces a new AAD B2C Directory to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Billing SKU for the B2C tenant. Must be one of: `PremiumP1` or `PremiumP2` (`Standard` is not supported). See [official docs](https://aka.ms/b2cBilling) for more information.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the AAD B2C Directory.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Tenant ID for the AAD B2C tenant.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DirectoryArgs,
    ) -> DirectoryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let country_code_binding_1 = args.country_code.get_output(context);
        let country_code_binding = country_code_binding_1.get_inner();
        let data_residency_location_binding_1 = args
            .data_residency_location
            .get_output(context);
        let data_residency_location_binding = data_residency_location_binding_1
            .get_inner();
        let display_name_binding_1 = args.display_name.get_output(context);
        let display_name_binding = display_name_binding_1.get_inner();
        let domain_name_binding_1 = args.domain_name.get_output(context);
        let domain_name_binding = domain_name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let sku_name_binding_1 = args.sku_name.get_output(context);
        let sku_name_binding = sku_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:aadb2c/directory:Directory".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "countryCode".into(),
                    value: &country_code_binding,
                },
                register_interface::ObjectField {
                    name: "dataResidencyLocation".into(),
                    value: &data_residency_location_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DirectoryResult {
            billing_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("billingType"),
            ),
            country_code: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("countryCode"),
            ),
            data_residency_location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataResidencyLocation"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            domain_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
            effective_start_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveStartDate"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tenant_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tenantId"),
            ),
        }
    }
}
