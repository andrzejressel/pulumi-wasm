#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_account_sas {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountSasArgs {
        /// The connection string for the storage account to which this SAS applies. Typically directly from the `primary_connection_string` attribute of a `azure.storage.Account` resource.
        #[builder(into)]
        pub connection_string: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The expiration time and date of this SAS. Must be a valid ISO-8601 format time/date string.
        ///
        /// > **NOTE:** The [ISO-8601 Time offset from UTC](https://en.wikipedia.org/wiki/ISO_8601#Time_offsets_from_UTC) is currently not supported by the service, which will result into 409 error.
        #[builder(into)]
        pub expiry: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Only permit `https` access. If `false`, both `http` and `https` are permitted. Defaults to `true`.
        #[builder(into, default)]
        pub https_only: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// IP address, or a range of IP addresses, from which to accept requests. When specifying a range, note that the range is inclusive.
        #[builder(into, default)]
        pub ip_addresses: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `permissions` block as defined below.
        #[builder(into)]
        pub permissions: pulumi_gestalt_rust::InputOrOutput<
            super::super::super::types::storage::GetAccountSasPermissions,
        >,
        /// A `resource_types` block as defined below.
        #[builder(into)]
        pub resource_types: pulumi_gestalt_rust::InputOrOutput<
            super::super::super::types::storage::GetAccountSasResourceTypes,
        >,
        /// A `services` block as defined below.
        #[builder(into)]
        pub services: pulumi_gestalt_rust::InputOrOutput<
            super::super::super::types::storage::GetAccountSasServices,
        >,
        /// Specifies the signed storage service version to use to authorize requests made with this account SAS. Defaults to `2017-07-29`.
        #[builder(into, default)]
        pub signed_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The starting time and date of validity of this SAS. Must be a valid ISO-8601 format time/date string.
        #[builder(into)]
        pub start: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccountSasResult {
        pub connection_string: pulumi_gestalt_rust::Output<String>,
        pub expiry: pulumi_gestalt_rust::Output<String>,
        pub https_only: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ip_addresses: pulumi_gestalt_rust::Output<Option<String>>,
        pub permissions: pulumi_gestalt_rust::Output<
            super::super::super::types::storage::GetAccountSasPermissions,
        >,
        pub resource_types: pulumi_gestalt_rust::Output<
            super::super::super::types::storage::GetAccountSasResourceTypes,
        >,
        /// The computed Account Shared Access Signature (SAS).
        pub sas: pulumi_gestalt_rust::Output<String>,
        pub services: pulumi_gestalt_rust::Output<
            super::super::super::types::storage::GetAccountSasServices,
        >,
        pub signed_version: pulumi_gestalt_rust::Output<Option<String>>,
        pub start: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAccountSasArgs,
    ) -> GetAccountSasResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let connection_string_binding_1 = args.connection_string.get_output(context);
        let connection_string_binding = connection_string_binding_1.get_inner();
        let expiry_binding_1 = args.expiry.get_output(context);
        let expiry_binding = expiry_binding_1.get_inner();
        let https_only_binding_1 = args.https_only.get_output(context);
        let https_only_binding = https_only_binding_1.get_inner();
        let ip_addresses_binding_1 = args.ip_addresses.get_output(context);
        let ip_addresses_binding = ip_addresses_binding_1.get_inner();
        let permissions_binding_1 = args.permissions.get_output(context);
        let permissions_binding = permissions_binding_1.get_inner();
        let resource_types_binding_1 = args.resource_types.get_output(context);
        let resource_types_binding = resource_types_binding_1.get_inner();
        let services_binding_1 = args.services.get_output(context);
        let services_binding = services_binding_1.get_inner();
        let signed_version_binding_1 = args.signed_version.get_output(context);
        let signed_version_binding = signed_version_binding_1.get_inner();
        let start_binding_1 = args.start.get_output(context);
        let start_binding = start_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:storage/getAccountSAS:getAccountSAS".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectionString".into(),
                    value: &connection_string_binding,
                },
                register_interface::ObjectField {
                    name: "expiry".into(),
                    value: &expiry_binding,
                },
                register_interface::ObjectField {
                    name: "httpsOnly".into(),
                    value: &https_only_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddresses".into(),
                    value: &ip_addresses_binding,
                },
                register_interface::ObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding,
                },
                register_interface::ObjectField {
                    name: "resourceTypes".into(),
                    value: &resource_types_binding,
                },
                register_interface::ObjectField {
                    name: "services".into(),
                    value: &services_binding,
                },
                register_interface::ObjectField {
                    name: "signedVersion".into(),
                    value: &signed_version_binding,
                },
                register_interface::ObjectField {
                    name: "start".into(),
                    value: &start_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAccountSasResult {
            connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionString"),
            ),
            expiry: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expiry"),
            ),
            https_only: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpsOnly"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddresses"),
            ),
            permissions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("permissions"),
            ),
            resource_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceTypes"),
            ),
            sas: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sas")),
            services: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("services"),
            ),
            signed_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("signedVersion"),
            ),
            start: pulumi_gestalt_rust::__private::into_domain(o.extract_field("start")),
        }
    }
}
