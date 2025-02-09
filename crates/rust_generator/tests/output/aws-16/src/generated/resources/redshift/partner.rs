/// Creates a new Amazon Redshift Partner Integration.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:redshift:Partner
///     properties:
///       clusterIdentifier: ${exampleAwsRedshiftCluster.id}
///       accountId: 1.23456791e+09
///       databaseName: ${exampleAwsRedshiftCluster.databaseName}
///       partnerName: example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift usage limits using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/partner:Partner example 01234567910:cluster-example-id:example:example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod partner {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PartnerArgs {
        /// The Amazon Web Services account ID that owns the cluster.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The cluster identifier of the cluster that receives data from the partner.
        #[builder(into)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the database that receives data from the partner.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the partner that is authorized to send data.
        #[builder(into)]
        pub partner_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PartnerResult {
        /// The Amazon Web Services account ID that owns the cluster.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The cluster identifier of the cluster that receives data from the partner.
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// The name of the database that receives data from the partner.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the partner that is authorized to send data.
        pub partner_name: pulumi_gestalt_rust::Output<String>,
        /// (Optional) The partner integration status.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// (Optional) The status message provided by the partner.
        pub status_message: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PartnerArgs,
    ) -> PartnerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding_1 = args.account_id.get_output(context);
        let account_id_binding = account_id_binding_1.get_inner();
        let cluster_identifier_binding_1 = args.cluster_identifier.get_output(context);
        let cluster_identifier_binding = cluster_identifier_binding_1.get_inner();
        let database_name_binding_1 = args.database_name.get_output(context);
        let database_name_binding = database_name_binding_1.get_inner();
        let partner_name_binding_1 = args.partner_name.get_output(context);
        let partner_name_binding = partner_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/partner:Partner".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
                },
                register_interface::ObjectField {
                    name: "partnerName".into(),
                    value: &partner_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PartnerResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            cluster_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterIdentifier"),
            ),
            database_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("databaseName"),
            ),
            partner_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("partnerName"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            status_message: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statusMessage"),
            ),
        }
    }
}
