#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_account_blob_container_sas {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountBlobContainerSasArgs {
        /// The `Cache-Control` response header that is sent when this SAS token is used.
        #[builder(into, default)]
        pub cache_control: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The connection string for the storage account to which this SAS applies. Typically directly from the `primary_connection_string` attribute of an `azure.storage.Account` resource.
        #[builder(into)]
        pub connection_string: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the container.
        #[builder(into)]
        pub container_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The `Content-Disposition` response header that is sent when this SAS token is used.
        #[builder(into, default)]
        pub content_disposition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The `Content-Encoding` response header that is sent when this SAS token is used.
        #[builder(into, default)]
        pub content_encoding: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The `Content-Language` response header that is sent when this SAS token is used.
        #[builder(into, default)]
        pub content_language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The `Content-Type` response header that is sent when this SAS token is used.
        #[builder(into, default)]
        pub content_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The expiration time and date of this SAS. Must be a valid ISO-8601 format time/date string.
        ///
        /// > **NOTE:** The [ISO-8601 Time offset from UTC](https://en.wikipedia.org/wiki/ISO_8601#Time_offsets_from_UTC) is currently not supported by the service, which will result into 409 error.
        #[builder(into)]
        pub expiry: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Only permit `https` access. If `false`, both `http` and `https` are permitted. Defaults to `true`.
        #[builder(into, default)]
        pub https_only: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Single IPv4 address or range (connected with a dash) of IPv4 addresses.
        #[builder(into, default)]
        pub ip_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `permissions` block as defined below.
        #[builder(into)]
        pub permissions: pulumi_gestalt_rust::InputOrOutput<
            super::super::super::types::storage::GetAccountBlobContainerSasPermissions,
        >,
        /// The starting time and date of validity of this SAS. Must be a valid ISO-8601 format time/date string.
        #[builder(into)]
        pub start: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccountBlobContainerSasResult {
        pub cache_control: pulumi_gestalt_rust::Output<Option<String>>,
        pub connection_string: pulumi_gestalt_rust::Output<String>,
        pub container_name: pulumi_gestalt_rust::Output<String>,
        pub content_disposition: pulumi_gestalt_rust::Output<Option<String>>,
        pub content_encoding: pulumi_gestalt_rust::Output<Option<String>>,
        pub content_language: pulumi_gestalt_rust::Output<Option<String>>,
        pub content_type: pulumi_gestalt_rust::Output<Option<String>>,
        pub expiry: pulumi_gestalt_rust::Output<String>,
        pub https_only: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ip_address: pulumi_gestalt_rust::Output<Option<String>>,
        pub permissions: pulumi_gestalt_rust::Output<
            super::super::super::types::storage::GetAccountBlobContainerSasPermissions,
        >,
        /// The computed Blob Container Shared Access Signature (SAS). The delimiter character ('?') for the query string is the prefix of `sas`.
        pub sas: pulumi_gestalt_rust::Output<String>,
        pub start: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAccountBlobContainerSasArgs,
    ) -> GetAccountBlobContainerSasResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cache_control_binding = args.cache_control.get_output(context);
        let connection_string_binding = args.connection_string.get_output(context);
        let container_name_binding = args.container_name.get_output(context);
        let content_disposition_binding = args.content_disposition.get_output(context);
        let content_encoding_binding = args.content_encoding.get_output(context);
        let content_language_binding = args.content_language.get_output(context);
        let content_type_binding = args.content_type.get_output(context);
        let expiry_binding = args.expiry.get_output(context);
        let https_only_binding = args.https_only.get_output(context);
        let ip_address_binding = args.ip_address.get_output(context);
        let permissions_binding = args.permissions.get_output(context);
        let start_binding = args.start.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:storage/getAccountBlobContainerSAS:getAccountBlobContainerSAS"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cacheControl".into(),
                    value: cache_control_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionString".into(),
                    value: connection_string_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerName".into(),
                    value: container_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentDisposition".into(),
                    value: content_disposition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentEncoding".into(),
                    value: content_encoding_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentLanguage".into(),
                    value: content_language_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentType".into(),
                    value: content_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expiry".into(),
                    value: expiry_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpsOnly".into(),
                    value: https_only_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipAddress".into(),
                    value: ip_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissions".into(),
                    value: permissions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "start".into(),
                    value: start_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAccountBlobContainerSasResult {
            cache_control: o.get_field("cacheControl"),
            connection_string: o.get_field("connectionString"),
            container_name: o.get_field("containerName"),
            content_disposition: o.get_field("contentDisposition"),
            content_encoding: o.get_field("contentEncoding"),
            content_language: o.get_field("contentLanguage"),
            content_type: o.get_field("contentType"),
            expiry: o.get_field("expiry"),
            https_only: o.get_field("httpsOnly"),
            id: o.get_field("id"),
            ip_address: o.get_field("ipAddress"),
            permissions: o.get_field("permissions"),
            sas: o.get_field("sas"),
            start: o.get_field("start"),
        }
    }
}
