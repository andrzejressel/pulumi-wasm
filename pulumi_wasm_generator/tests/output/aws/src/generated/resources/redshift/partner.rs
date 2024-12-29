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
pub mod partner {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PartnerArgs {
        /// The Amazon Web Services account ID that owns the cluster.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The cluster identifier of the cluster that receives data from the partner.
        #[builder(into)]
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// The name of the database that receives data from the partner.
        #[builder(into)]
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// The name of the partner that is authorized to send data.
        #[builder(into)]
        pub partner_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PartnerResult {
        /// The Amazon Web Services account ID that owns the cluster.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The cluster identifier of the cluster that receives data from the partner.
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// The name of the database that receives data from the partner.
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// The name of the partner that is authorized to send data.
        pub partner_name: pulumi_wasm_rust::Output<String>,
        /// (Optional) The partner integration status.
        pub status: pulumi_wasm_rust::Output<String>,
        /// (Optional) The status message provided by the partner.
        pub status_message: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PartnerArgs) -> PartnerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let cluster_identifier_binding = args.cluster_identifier.get_inner();
        let database_name_binding = args.database_name.get_inner();
        let partner_name_binding = args.partner_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/partner:Partner".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "clusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "partnerName".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "statusMessage".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PartnerResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterIdentifier").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            partner_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partnerName").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            status_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusMessage").unwrap(),
            ),
        }
    }
}
