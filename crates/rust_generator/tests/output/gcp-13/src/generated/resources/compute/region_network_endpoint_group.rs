/// A regional NEG that can support Serverless Products, proxying traffic to
/// external backends and providing traffic to the PSC port mapping endpoints.
///
/// To get more information about RegionNetworkEndpointGroup, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/beta/regionNetworkEndpointGroups)
/// * How-to Guides
///     * [Internet NEGs Official Documentation](https://cloud.google.com/load-balancing/docs/negs/internet-neg-concepts)
///     * [Serverless NEGs Official Documentation](https://cloud.google.com/load-balancing/docs/negs/serverless-neg-concepts)
///
/// ## Example Usage
///
/// ### Region Network Endpoint Group Functions
///
///
/// ```yaml
/// resources:
///   # Cloud Functions Example
///   functionNeg:
///     type: gcp:compute:RegionNetworkEndpointGroup
///     name: function_neg
///     properties:
///       name: function-neg
///       networkEndpointType: SERVERLESS
///       region: us-central1
///       cloudFunction:
///         function: ${functionNegFunction.name}
///   functionNegFunction:
///     type: gcp:cloudfunctions:Function
///     name: function_neg
///     properties:
///       name: function-neg
///       description: My function
///       runtime: nodejs10
///       availableMemoryMb: 128
///       sourceArchiveBucket: ${bucket.name}
///       sourceArchiveObject: ${archive.name}
///       triggerHttp: true
///       timeout: 60
///       entryPoint: helloGET
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: cloudfunctions-function-example-bucket
///       location: US
///   archive:
///     type: gcp:storage:BucketObject
///     properties:
///       name: index.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: path/to/index.zip
/// ```
/// ### Region Network Endpoint Group Cloudrun
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cloudrunNeg = region_network_endpoint_group::create(
///         "cloudrunNeg",
///         RegionNetworkEndpointGroupArgs::builder()
///             .cloud_run(
///                 RegionNetworkEndpointGroupCloudRun::builder()
///                     .service("${cloudrunNegService.name}")
///                     .build_struct(),
///             )
///             .name("cloudrun-neg")
///             .network_endpoint_type("SERVERLESS")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let cloudrunNegService = service::create(
///         "cloudrunNegService",
///         ServiceArgs::builder()
///             .location("us-central1")
///             .name("cloudrun-neg")
///             .template(
///                 ServiceTemplate::builder()
///                     .spec(
///                         ServiceTemplateSpec::builder()
///                             .containers(
///                                 vec![
///                                     ServiceTemplateSpecContainer::builder()
///                                     .image("us-docker.pkg.dev/cloudrun/container/hello")
///                                     .build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .traffics(
///                 vec![
///                     ServiceTraffic::builder().latestRevision(true).percent(100)
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Network Endpoint Group Appengine
///
///
/// ```yaml
/// resources:
///   # App Engine Example
///   appengineNeg:
///     type: gcp:compute:RegionNetworkEndpointGroup
///     name: appengine_neg
///     properties:
///       name: appengine-neg
///       networkEndpointType: SERVERLESS
///       region: us-central1
///       appEngine:
///         service: ${appengineNegFlexibleAppVersion.service}
///         version: ${appengineNegFlexibleAppVersion.versionId}
///   appengineNegFlexibleAppVersion:
///     type: gcp:appengine:FlexibleAppVersion
///     name: appengine_neg
///     properties:
///       versionId: v1
///       service: appengine-neg
///       runtime: nodejs
///       flexibleRuntimeSettings:
///         operatingSystem: ubuntu22
///         runtimeVersion: '20'
///       entrypoint:
///         shell: node ./app.js
///       deployment:
///         zip:
///           sourceUrl: https://storage.googleapis.com/${appengineNegBucket.name}/${appengineNegBucketObject.name}
///       livenessCheck:
///         path: /
///       readinessCheck:
///         path: /
///       envVariables:
///         port: '8080'
///       handlers:
///         - urlRegex: .*\/my-path\/*
///           securityLevel: SECURE_ALWAYS
///           login: LOGIN_REQUIRED
///           authFailAction: AUTH_FAIL_ACTION_REDIRECT
///           staticFiles:
///             path: my-other-path
///             uploadPathRegex: .*\/my-path\/*
///       automaticScaling:
///         coolDownPeriod: 120s
///         cpuUtilization:
///           targetUtilization: 0.5
///       deleteServiceOnDestroy: true
///   appengineNegBucket:
///     type: gcp:storage:Bucket
///     name: appengine_neg
///     properties:
///       name: appengine-neg
///       location: US
///       uniformBucketLevelAccess: true
///   appengineNegBucketObject:
///     type: gcp:storage:BucketObject
///     name: appengine_neg
///     properties:
///       name: hello-world.zip
///       bucket: ${appengineNegBucket.name}
///       source:
///         fn::FileAsset: ./test-fixtures/hello-world.zip
/// ```
/// ### Region Network Endpoint Group Appengine Empty
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let appengineNeg = region_network_endpoint_group::create(
///         "appengineNeg",
///         RegionNetworkEndpointGroupArgs::builder()
///             .app_engine(RegionNetworkEndpointGroupAppEngine::builder().build_struct())
///             .name("appengine-neg")
///             .network_endpoint_type("SERVERLESS")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Network Endpoint Group Psc
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let pscNeg = region_network_endpoint_group::create(
///         "pscNeg",
///         RegionNetworkEndpointGroupArgs::builder()
///             .name("psc-neg")
///             .network_endpoint_type("PRIVATE_SERVICE_CONNECT")
///             .psc_target_service("asia-northeast3-cloudkms.googleapis.com")
///             .region("asia-northeast3")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Network Endpoint Group Psc Service Attachment
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: psc-network
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: psc-subnetwork
///       ipCidrRange: 10.0.0.0/16
///       region: europe-west4
///       network: ${default.id}
///   pscSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: psc_subnetwork
///     properties:
///       name: psc-subnetwork-nat
///       ipCidrRange: 10.1.0.0/16
///       region: europe-west4
///       purpose: PRIVATE_SERVICE_CONNECT
///       network: ${default.id}
///   defaultHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: default
///     properties:
///       name: psc-healthcheck
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///   defaultRegionBackendService:
///     type: gcp:compute:RegionBackendService
///     name: default
///     properties:
///       name: psc-backend
///       region: europe-west4
///       healthChecks: ${defaultHealthCheck.id}
///   defaultForwardingRule:
///     type: gcp:compute:ForwardingRule
///     name: default
///     properties:
///       name: psc-forwarding-rule
///       region: europe-west4
///       loadBalancingScheme: INTERNAL
///       backendService: ${defaultRegionBackendService.id}
///       ports:
///         - '80'
///         - '88'
///         - '443'
///       network: ${default.name}
///       subnetwork: ${defaultSubnetwork.name}
///   defaultServiceAttachment:
///     type: gcp:compute:ServiceAttachment
///     name: default
///     properties:
///       name: psc-service-attachment
///       region: europe-west4
///       description: A service attachment configured with Terraform
///       enableProxyProtocol: false
///       connectionPreference: ACCEPT_AUTOMATIC
///       natSubnets:
///         - ${pscSubnetwork.selfLink}
///       targetService: ${defaultForwardingRule.selfLink}
///   pscNegServiceAttachment:
///     type: gcp:compute:RegionNetworkEndpointGroup
///     name: psc_neg_service_attachment
///     properties:
///       name: psc-neg
///       region: europe-west4
///       networkEndpointType: PRIVATE_SERVICE_CONNECT
///       pscTargetService: ${defaultServiceAttachment.selfLink}
///       pscData:
///         producerPort: '88'
///       network: ${default.selfLink}
///       subnetwork: ${defaultSubnetwork.selfLink}
/// ```
/// ### Region Network Endpoint Group Internet Ip Port
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = network::create(
///         "default",
///         NetworkArgs::builder().name("network").build_struct(),
///     );
///     let regionNetworkEndpointGroupInternetIpPort = region_network_endpoint_group::create(
///         "regionNetworkEndpointGroupInternetIpPort",
///         RegionNetworkEndpointGroupArgs::builder()
///             .name("ip-port-neg")
///             .network("${default.id}")
///             .network_endpoint_type("INTERNET_IP_PORT")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Network Endpoint Group Internet Fqdn Port
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = network::create(
///         "default",
///         NetworkArgs::builder().name("network").build_struct(),
///     );
///     let regionNetworkEndpointGroupInternetFqdnPort = region_network_endpoint_group::create(
///         "regionNetworkEndpointGroupInternetFqdnPort",
///         RegionNetworkEndpointGroupArgs::builder()
///             .name("ip-port-neg")
///             .network("${default.id}")
///             .network_endpoint_type("INTERNET_FQDN_PORT")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Network Endpoint Group Portmap
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = network::create(
///         "default",
///         NetworkArgs::builder().name("network").build_struct(),
///     );
///     let defaultSubnetwork = subnetwork::create(
///         "defaultSubnetwork",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.0.0/16")
///             .name("subnetwork")
///             .network("${default.id}")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let regionNetworkEndpointGroupPortmap = region_network_endpoint_group::create(
///         "regionNetworkEndpointGroupPortmap",
///         RegionNetworkEndpointGroupArgs::builder()
///             .name("portmap-neg")
///             .network("${default.id}")
///             .network_endpoint_type("GCE_VM_IP_PORTMAP")
///             .region("us-central1")
///             .subnetwork("${defaultSubnetwork.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// RegionNetworkEndpointGroup can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/networkEndpointGroups/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, RegionNetworkEndpointGroup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkEndpointGroup:RegionNetworkEndpointGroup default projects/{{project}}/regions/{{region}}/networkEndpointGroups/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkEndpointGroup:RegionNetworkEndpointGroup default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkEndpointGroup:RegionNetworkEndpointGroup default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkEndpointGroup:RegionNetworkEndpointGroup default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod region_network_endpoint_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionNetworkEndpointGroupArgs {
        /// This field is only used for SERVERLESS NEGs.
        /// Only one of cloud_run, app_engine, cloud_function or serverless_deployment may be set.
        /// Structure is documented below.
        #[builder(into, default)]
        pub app_engine: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionNetworkEndpointGroupAppEngine>,
        >,
        /// This field is only used for SERVERLESS NEGs.
        /// Only one of cloud_run, app_engine, cloud_function or serverless_deployment may be set.
        /// Structure is documented below.
        #[builder(into, default)]
        pub cloud_function: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionNetworkEndpointGroupCloudFunction>,
        >,
        /// This field is only used for SERVERLESS NEGs.
        /// Only one of cloud_run, app_engine, cloud_function or serverless_deployment may be set.
        /// Structure is documented below.
        #[builder(into, default)]
        pub cloud_run: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionNetworkEndpointGroupCloudRun>,
        >,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the resource; provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This field is only used for PSC and INTERNET NEGs.
        /// The URL of the network to which all network endpoints in the NEG belong. Uses
        /// "default" project network if unspecified.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Type of network endpoints in this network endpoint group. Defaults to SERVERLESS.
        /// Default value is `SERVERLESS`.
        /// Possible values are: `SERVERLESS`, `PRIVATE_SERVICE_CONNECT`, `INTERNET_IP_PORT`, `INTERNET_FQDN_PORT`, `GCE_VM_IP_PORTMAP`.
        #[builder(into, default)]
        pub network_endpoint_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This field is only used for PSC NEGs.
        /// Structure is documented below.
        #[builder(into, default)]
        pub psc_data: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionNetworkEndpointGroupPscData>,
        >,
        /// This field is only used for PSC and INTERNET NEGs.
        /// The target service url used to set up private service connection to
        /// a Google API or a PSC Producer Service Attachment.
        #[builder(into, default)]
        pub psc_target_service: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the region where the regional NEGs reside.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub region: pulumi_gestalt_rust::InputOrOutput<String>,
        /// This field is only used for SERVERLESS NEGs.
        /// Only one of cloudRun, appEngine, cloudFunction or serverlessDeployment may be set.
        /// Structure is documented below.
        #[builder(into, default)]
        pub serverless_deployment: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::RegionNetworkEndpointGroupServerlessDeployment,
            >,
        >,
        /// This field is only used for PSC NEGs.
        /// Optional URL of the subnetwork to which all network endpoints in the NEG belong.
        #[builder(into, default)]
        pub subnetwork: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RegionNetworkEndpointGroupResult {
        /// This field is only used for SERVERLESS NEGs.
        /// Only one of cloud_run, app_engine, cloud_function or serverless_deployment may be set.
        /// Structure is documented below.
        pub app_engine: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionNetworkEndpointGroupAppEngine>,
        >,
        /// This field is only used for SERVERLESS NEGs.
        /// Only one of cloud_run, app_engine, cloud_function or serverless_deployment may be set.
        /// Structure is documented below.
        pub cloud_function: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionNetworkEndpointGroupCloudFunction>,
        >,
        /// This field is only used for SERVERLESS NEGs.
        /// Only one of cloud_run, app_engine, cloud_function or serverless_deployment may be set.
        /// Structure is documented below.
        pub cloud_run: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionNetworkEndpointGroupCloudRun>,
        >,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the resource; provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// This field is only used for PSC and INTERNET NEGs.
        /// The URL of the network to which all network endpoints in the NEG belong. Uses
        /// "default" project network if unspecified.
        pub network: pulumi_gestalt_rust::Output<Option<String>>,
        /// Type of network endpoints in this network endpoint group. Defaults to SERVERLESS.
        /// Default value is `SERVERLESS`.
        /// Possible values are: `SERVERLESS`, `PRIVATE_SERVICE_CONNECT`, `INTERNET_IP_PORT`, `INTERNET_FQDN_PORT`, `GCE_VM_IP_PORTMAP`.
        pub network_endpoint_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// This field is only used for PSC NEGs.
        /// Structure is documented below.
        pub psc_data: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionNetworkEndpointGroupPscData>,
        >,
        /// This field is only used for PSC and INTERNET NEGs.
        /// The target service url used to set up private service connection to
        /// a Google API or a PSC Producer Service Attachment.
        pub psc_target_service: pulumi_gestalt_rust::Output<Option<String>>,
        /// A reference to the region where the regional NEGs reside.
        ///
        ///
        /// - - -
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// This field is only used for SERVERLESS NEGs.
        /// Only one of cloudRun, appEngine, cloudFunction or serverlessDeployment may be set.
        /// Structure is documented below.
        pub serverless_deployment: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::RegionNetworkEndpointGroupServerlessDeployment,
            >,
        >,
        /// This field is only used for PSC NEGs.
        /// Optional URL of the subnetwork to which all network endpoints in the NEG belong.
        pub subnetwork: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegionNetworkEndpointGroupArgs,
    ) -> RegionNetworkEndpointGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_engine_binding = args.app_engine.get_output(context);
        let cloud_function_binding = args.cloud_function.get_output(context);
        let cloud_run_binding = args.cloud_run.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let network_endpoint_type_binding = args
            .network_endpoint_type
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let psc_data_binding = args.psc_data.get_output(context);
        let psc_target_service_binding = args.psc_target_service.get_output(context);
        let region_binding = args.region.get_output(context);
        let serverless_deployment_binding = args
            .serverless_deployment
            .get_output(context);
        let subnetwork_binding = args.subnetwork.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/regionNetworkEndpointGroup:RegionNetworkEndpointGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appEngine".into(),
                    value: app_engine_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudFunction".into(),
                    value: cloud_function_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudRun".into(),
                    value: cloud_run_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: network_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkEndpointType".into(),
                    value: network_endpoint_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pscData".into(),
                    value: psc_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pscTargetService".into(),
                    value: psc_target_service_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverlessDeployment".into(),
                    value: serverless_deployment_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetwork".into(),
                    value: subnetwork_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegionNetworkEndpointGroupResult {
            app_engine: o.get_field("appEngine"),
            cloud_function: o.get_field("cloudFunction"),
            cloud_run: o.get_field("cloudRun"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            network_endpoint_type: o.get_field("networkEndpointType"),
            project: o.get_field("project"),
            psc_data: o.get_field("pscData"),
            psc_target_service: o.get_field("pscTargetService"),
            region: o.get_field("region"),
            self_link: o.get_field("selfLink"),
            serverless_deployment: o.get_field("serverlessDeployment"),
            subnetwork: o.get_field("subnetwork"),
        }
    }
}
