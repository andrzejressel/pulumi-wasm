/// A ForwardingRule resource. A ForwardingRule resource specifies which pool
/// of target virtual machines to forward a packet to if it matches the given
/// [IPAddress, IPProtocol, portRange] tuple.
///
///
/// To get more information about ForwardingRule, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/v1/forwardingRules)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/load-balancing/network/forwarding-rules)
///
/// ## Example Usage
///
/// ### Forwarding Rule Externallb
///
///
/// ```yaml
/// resources:
///   # Forwarding rule for External Network Load Balancing using Backend Services
///   default:
///     type: gcp:compute:ForwardingRule
///     properties:
///       name: website-forwarding-rule
///       region: us-central1
///       portRange: 80
///       backendService: ${backend.id}
///   backend:
///     type: gcp:compute:RegionBackendService
///     properties:
///       name: website-backend
///       region: us-central1
///       loadBalancingScheme: EXTERNAL
///       healthChecks: ${hc.id}
///   hc:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       name: check-website-backend
///       checkIntervalSec: 1
///       timeoutSec: 1
///       region: us-central1
///       tcpHealthCheck:
///         port: '80'
/// ```
/// ### Forwarding Rule Global Internallb
///
///
/// ```yaml
/// resources:
///   # Forwarding rule for Internal Load Balancing
///   default:
///     type: gcp:compute:ForwardingRule
///     properties:
///       name: website-forwarding-rule
///       region: us-central1
///       loadBalancingScheme: INTERNAL
///       backendService: ${backend.id}
///       allPorts: true
///       allowGlobalAccess: true
///       network: ${defaultNetwork.name}
///       subnetwork: ${defaultSubnetwork.name}
///   backend:
///     type: gcp:compute:RegionBackendService
///     properties:
///       name: website-backend
///       region: us-central1
///       healthChecks: ${hc.id}
///   hc:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: check-website-backend
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: website-net
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: website-net
///       ipCidrRange: 10.0.0.0/16
///       region: us-central1
///       network: ${defaultNetwork.id}
/// ```
/// ### Forwarding Rule Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = forwarding_rule::create(
///         "default",
///         ForwardingRuleArgs::builder()
///             .name("website-forwarding-rule")
///             .port_range("80")
///             .target("${defaultTargetPool.id}")
///             .build_struct(),
///     );
///     let defaultTargetPool = target_pool::create(
///         "defaultTargetPool",
///         TargetPoolArgs::builder().name("website-target-pool").build_struct(),
///     );
/// }
/// ```
/// ### Forwarding Rule L3 Default
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let fwdRule = forwarding_rule::create(
///         "fwdRule",
///         ForwardingRuleArgs::builder()
///             .all_ports(true)
///             .backend_service("${service.id}")
///             .ip_protocol("L3_DEFAULT")
///             .name("l3-forwarding-rule")
///             .build_struct(),
///     );
///     let healthCheck = region_health_check::create(
///         "healthCheck",
///         RegionHealthCheckArgs::builder()
///             .name("health-check")
///             .region("us-central1")
///             .tcp_health_check(
///                 RegionHealthCheckTcpHealthCheck::builder().port(80).build_struct(),
///             )
///             .build_struct(),
///     );
///     let service = region_backend_service::create(
///         "service",
///         RegionBackendServiceArgs::builder()
///             .health_checks("${healthCheck.id}")
///             .load_balancing_scheme("EXTERNAL")
///             .name("service")
///             .protocol("UNSPECIFIED")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Forwarding Rule Internallb
///
///
/// ```yaml
/// resources:
///   # Forwarding rule for Internal Load Balancing
///   default:
///     type: gcp:compute:ForwardingRule
///     properties:
///       name: website-forwarding-rule
///       region: us-central1
///       loadBalancingScheme: INTERNAL
///       backendService: ${backend.id}
///       allPorts: true
///       network: ${defaultNetwork.name}
///       subnetwork: ${defaultSubnetwork.name}
///       ipVersion: IPV4
///   backend:
///     type: gcp:compute:RegionBackendService
///     properties:
///       name: website-backend
///       region: us-central1
///       healthChecks: ${hc.id}
///   hc:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: check-website-backend
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: website-net
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: website-net
///       ipCidrRange: 10.0.0.0/16
///       region: us-central1
///       network: ${defaultNetwork.id}
/// ```
/// ### Forwarding Rule Http Lb
///
///
/// ```yaml
/// resources:
///   # Forwarding rule for Internal Load Balancing
///   default:
///     type: gcp:compute:ForwardingRule
///     properties:
///       name: website-forwarding-rule
///       region: us-central1
///       ipProtocol: TCP
///       loadBalancingScheme: INTERNAL_MANAGED
///       portRange: '80'
///       target: ${defaultRegionTargetHttpProxy.id}
///       network: ${defaultNetwork.id}
///       subnetwork: ${defaultSubnetwork.id}
///       networkTier: PREMIUM
///     options:
///       dependsOn:
///         - ${proxy}
///   defaultRegionTargetHttpProxy:
///     type: gcp:compute:RegionTargetHttpProxy
///     name: default
///     properties:
///       region: us-central1
///       name: website-proxy
///       urlMap: ${defaultRegionUrlMap.id}
///   defaultRegionUrlMap:
///     type: gcp:compute:RegionUrlMap
///     name: default
///     properties:
///       region: us-central1
///       name: website-map
///       defaultService: ${defaultRegionBackendService.id}
///   defaultRegionBackendService:
///     type: gcp:compute:RegionBackendService
///     name: default
///     properties:
///       loadBalancingScheme: INTERNAL_MANAGED
///       backends:
///         - group: ${rigm.instanceGroup}
///           balancingMode: UTILIZATION
///           capacityScaler: 1
///       region: us-central1
///       name: website-backend
///       protocol: HTTP
///       timeoutSec: 10
///       healthChecks: ${defaultRegionHealthCheck.id}
///   rigm:
///     type: gcp:compute:RegionInstanceGroupManager
///     properties:
///       region: us-central1
///       name: website-rigm
///       versions:
///         - instanceTemplate: ${instanceTemplate.id}
///           name: primary
///       baseInstanceName: internal-glb
///       targetSize: 1
///   instanceTemplate:
///     type: gcp:compute:InstanceTemplate
///     name: instance_template
///     properties:
///       name: template-website-backend
///       machineType: e2-medium
///       networkInterfaces:
///         - network: ${defaultNetwork.id}
///           subnetwork: ${defaultSubnetwork.id}
///       disks:
///         - sourceImage: ${debianImage.selfLink}
///           autoDelete: true
///           boot: true
///       tags:
///         - allow-ssh
///         - load-balanced-backend
///   defaultRegionHealthCheck:
///     type: gcp:compute:RegionHealthCheck
///     name: default
///     properties:
///       region: us-central1
///       name: website-hc
///       httpHealthCheck:
///         portSpecification: USE_SERVING_PORT
///     options:
///       dependsOn:
///         - ${fw4}
///   fw1:
///     type: gcp:compute:Firewall
///     properties:
///       name: website-fw-1
///       network: ${defaultNetwork.id}
///       sourceRanges:
///         - 10.1.2.0/24
///       allows:
///         - protocol: tcp
///         - protocol: udp
///         - protocol: icmp
///       direction: INGRESS
///   fw2:
///     type: gcp:compute:Firewall
///     properties:
///       name: website-fw-2
///       network: ${defaultNetwork.id}
///       sourceRanges:
///         - 0.0.0.0/0
///       allows:
///         - protocol: tcp
///           ports:
///             - '22'
///       targetTags:
///         - allow-ssh
///       direction: INGRESS
///     options:
///       dependsOn:
///         - ${fw1}
///   fw3:
///     type: gcp:compute:Firewall
///     properties:
///       name: website-fw-3
///       network: ${defaultNetwork.id}
///       sourceRanges:
///         - 130.211.0.0/22
///         - 35.191.0.0/16
///       allows:
///         - protocol: tcp
///       targetTags:
///         - load-balanced-backend
///       direction: INGRESS
///     options:
///       dependsOn:
///         - ${fw2}
///   fw4:
///     type: gcp:compute:Firewall
///     properties:
///       name: website-fw-4
///       network: ${defaultNetwork.id}
///       sourceRanges:
///         - 10.129.0.0/26
///       targetTags:
///         - load-balanced-backend
///       allows:
///         - protocol: tcp
///           ports:
///             - '80'
///         - protocol: tcp
///           ports:
///             - '443'
///         - protocol: tcp
///           ports:
///             - '8000'
///       direction: INGRESS
///     options:
///       dependsOn:
///         - ${fw3}
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: website-net
///       autoCreateSubnetworks: false
///       routingMode: REGIONAL
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: website-net-default
///       ipCidrRange: 10.1.2.0/24
///       region: us-central1
///       network: ${defaultNetwork.id}
///   proxy:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: website-net-proxy
///       ipCidrRange: 10.129.0.0/26
///       region: us-central1
///       network: ${defaultNetwork.id}
///       purpose: REGIONAL_MANAGED_PROXY
///       role: ACTIVE
/// variables:
///   debianImage:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-11
///         project: debian-cloud
/// ```
/// ### Forwarding Rule Regional Http Xlb
///
///
/// ```yaml
/// resources:
///   # Forwarding rule for Regional External Load Balancing
///   default:
///     type: gcp:compute:ForwardingRule
///     properties:
///       name: website-forwarding-rule
///       region: us-central1
///       ipProtocol: TCP
///       loadBalancingScheme: EXTERNAL_MANAGED
///       portRange: '80'
///       target: ${defaultRegionTargetHttpProxy.id}
///       network: ${defaultNetwork.id}
///       ipAddress: ${defaultAddress.address}
///       networkTier: STANDARD
///     options:
///       dependsOn:
///         - ${proxy}
///   defaultRegionTargetHttpProxy:
///     type: gcp:compute:RegionTargetHttpProxy
///     name: default
///     properties:
///       region: us-central1
///       name: website-proxy
///       urlMap: ${defaultRegionUrlMap.id}
///   defaultRegionUrlMap:
///     type: gcp:compute:RegionUrlMap
///     name: default
///     properties:
///       region: us-central1
///       name: website-map
///       defaultService: ${defaultRegionBackendService.id}
///   defaultRegionBackendService:
///     type: gcp:compute:RegionBackendService
///     name: default
///     properties:
///       loadBalancingScheme: EXTERNAL_MANAGED
///       backends:
///         - group: ${rigm.instanceGroup}
///           balancingMode: UTILIZATION
///           capacityScaler: 1
///       region: us-central1
///       name: website-backend
///       protocol: HTTP
///       timeoutSec: 10
///       healthChecks: ${defaultRegionHealthCheck.id}
///   rigm:
///     type: gcp:compute:RegionInstanceGroupManager
///     properties:
///       region: us-central1
///       name: website-rigm
///       versions:
///         - instanceTemplate: ${instanceTemplate.id}
///           name: primary
///       baseInstanceName: internal-glb
///       targetSize: 1
///   instanceTemplate:
///     type: gcp:compute:InstanceTemplate
///     name: instance_template
///     properties:
///       name: template-website-backend
///       machineType: e2-medium
///       networkInterfaces:
///         - network: ${defaultNetwork.id}
///           subnetwork: ${defaultSubnetwork.id}
///       disks:
///         - sourceImage: ${debianImage.selfLink}
///           autoDelete: true
///           boot: true
///       tags:
///         - allow-ssh
///         - load-balanced-backend
///   defaultRegionHealthCheck:
///     type: gcp:compute:RegionHealthCheck
///     name: default
///     properties:
///       region: us-central1
///       name: website-hc
///       httpHealthCheck:
///         portSpecification: USE_SERVING_PORT
///     options:
///       dependsOn:
///         - ${fw4}
///   defaultAddress:
///     type: gcp:compute:Address
///     name: default
///     properties:
///       name: website-ip-1
///       region: us-central1
///       networkTier: STANDARD
///   fw1:
///     type: gcp:compute:Firewall
///     properties:
///       name: website-fw-1
///       network: ${defaultNetwork.id}
///       sourceRanges:
///         - 10.1.2.0/24
///       allows:
///         - protocol: tcp
///         - protocol: udp
///         - protocol: icmp
///       direction: INGRESS
///   fw2:
///     type: gcp:compute:Firewall
///     properties:
///       name: website-fw-2
///       network: ${defaultNetwork.id}
///       sourceRanges:
///         - 0.0.0.0/0
///       allows:
///         - protocol: tcp
///           ports:
///             - '22'
///       targetTags:
///         - allow-ssh
///       direction: INGRESS
///     options:
///       dependsOn:
///         - ${fw1}
///   fw3:
///     type: gcp:compute:Firewall
///     properties:
///       name: website-fw-3
///       network: ${defaultNetwork.id}
///       sourceRanges:
///         - 130.211.0.0/22
///         - 35.191.0.0/16
///       allows:
///         - protocol: tcp
///       targetTags:
///         - load-balanced-backend
///       direction: INGRESS
///     options:
///       dependsOn:
///         - ${fw2}
///   fw4:
///     type: gcp:compute:Firewall
///     properties:
///       name: website-fw-4
///       network: ${defaultNetwork.id}
///       sourceRanges:
///         - 10.129.0.0/26
///       targetTags:
///         - load-balanced-backend
///       allows:
///         - protocol: tcp
///           ports:
///             - '80'
///         - protocol: tcp
///           ports:
///             - '443'
///         - protocol: tcp
///           ports:
///             - '8000'
///       direction: INGRESS
///     options:
///       dependsOn:
///         - ${fw3}
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: website-net
///       autoCreateSubnetworks: false
///       routingMode: REGIONAL
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: website-net-default
///       ipCidrRange: 10.1.2.0/24
///       region: us-central1
///       network: ${defaultNetwork.id}
///   proxy:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: website-net-proxy
///       ipCidrRange: 10.129.0.0/26
///       region: us-central1
///       network: ${defaultNetwork.id}
///       purpose: REGIONAL_MANAGED_PROXY
///       role: ACTIVE
/// variables:
///   debianImage:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-11
///         project: debian-cloud
/// ```
/// ### Forwarding Rule Vpc Psc
///
///
/// ```yaml
/// resources:
///   # Forwarding rule for VPC private service connect
///   default:
///     type: gcp:compute:ForwardingRule
///     properties:
///       name: psc-endpoint
///       region: us-central1
///       loadBalancingScheme: ""
///       target: ${producerServiceAttachment.id}
///       network: ${consumerNet.name}
///       ipAddress: ${consumerAddress.id}
///       allowPscGlobalAccess: true
///   # Consumer service endpoint
///   consumerNet:
///     type: gcp:compute:Network
///     name: consumer_net
///     properties:
///       name: consumer-net
///       autoCreateSubnetworks: false
///   consumerSubnet:
///     type: gcp:compute:Subnetwork
///     name: consumer_subnet
///     properties:
///       name: consumer-net
///       ipCidrRange: 10.0.0.0/16
///       region: us-central1
///       network: ${consumerNet.id}
///   consumerAddress:
///     type: gcp:compute:Address
///     name: consumer_address
///     properties:
///       name: website-ip-1
///       region: us-central1
///       subnetwork: ${consumerSubnet.id}
///       addressType: INTERNAL
///   # Producer service attachment
///   producerNet:
///     type: gcp:compute:Network
///     name: producer_net
///     properties:
///       name: producer-net
///       autoCreateSubnetworks: false
///   producerSubnet:
///     type: gcp:compute:Subnetwork
///     name: producer_subnet
///     properties:
///       name: producer-net
///       ipCidrRange: 10.0.0.0/16
///       region: us-central1
///       network: ${producerNet.id}
///   pscProducerSubnet:
///     type: gcp:compute:Subnetwork
///     name: psc_producer_subnet
///     properties:
///       name: producer-psc-net
///       ipCidrRange: 10.1.0.0/16
///       region: us-central1
///       purpose: PRIVATE_SERVICE_CONNECT
///       network: ${producerNet.id}
///   producerServiceAttachment:
///     type: gcp:compute:ServiceAttachment
///     name: producer_service_attachment
///     properties:
///       name: producer-service
///       region: us-central1
///       description: A service attachment configured with Terraform
///       enableProxyProtocol: true
///       connectionPreference: ACCEPT_AUTOMATIC
///       natSubnets:
///         - ${pscProducerSubnet.name}
///       targetService: ${producerTargetService.id}
///   producerTargetService:
///     type: gcp:compute:ForwardingRule
///     name: producer_target_service
///     properties:
///       name: producer-forwarding-rule
///       region: us-central1
///       loadBalancingScheme: INTERNAL
///       backendService: ${producerServiceBackend.id}
///       allPorts: true
///       network: ${producerNet.name}
///       subnetwork: ${producerSubnet.name}
///   producerServiceBackend:
///     type: gcp:compute:RegionBackendService
///     name: producer_service_backend
///     properties:
///       name: producer-service-backend
///       region: us-central1
///       healthChecks: ${producerServiceHealthCheck.id}
///   producerServiceHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: producer_service_health_check
///     properties:
///       name: producer-service-health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
/// ```
/// ### Forwarding Rule Vpc Psc No Automate Dns
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:ForwardingRule
///     properties:
///       name: psc-endpoint
///       region: us-central1
///       loadBalancingScheme: ""
///       target: ${producerServiceAttachment.id}
///       network: ${consumerNet.name}
///       ipAddress: ${consumerAddress.id}
///       allowPscGlobalAccess: true
///       noAutomateDnsZone: true
///   consumerNet:
///     type: gcp:compute:Network
///     name: consumer_net
///     properties:
///       name: consumer-net
///       autoCreateSubnetworks: false
///   consumerSubnet:
///     type: gcp:compute:Subnetwork
///     name: consumer_subnet
///     properties:
///       name: consumer-net
///       ipCidrRange: 10.0.0.0/16
///       region: us-central1
///       network: ${consumerNet.id}
///   consumerAddress:
///     type: gcp:compute:Address
///     name: consumer_address
///     properties:
///       name: website-ip-1
///       region: us-central1
///       subnetwork: ${consumerSubnet.id}
///       addressType: INTERNAL
///   producerNet:
///     type: gcp:compute:Network
///     name: producer_net
///     properties:
///       name: producer-net
///       autoCreateSubnetworks: false
///   producerSubnet:
///     type: gcp:compute:Subnetwork
///     name: producer_subnet
///     properties:
///       name: producer-net
///       ipCidrRange: 10.0.0.0/16
///       region: us-central1
///       network: ${producerNet.id}
///   pscProducerSubnet:
///     type: gcp:compute:Subnetwork
///     name: psc_producer_subnet
///     properties:
///       name: producer-psc-net
///       ipCidrRange: 10.1.0.0/16
///       region: us-central1
///       purpose: PRIVATE_SERVICE_CONNECT
///       network: ${producerNet.id}
///   producerServiceAttachment:
///     type: gcp:compute:ServiceAttachment
///     name: producer_service_attachment
///     properties:
///       name: producer-service
///       region: us-central1
///       description: A service attachment configured with Terraform
///       enableProxyProtocol: true
///       connectionPreference: ACCEPT_AUTOMATIC
///       natSubnets:
///         - ${pscProducerSubnet.name}
///       targetService: ${producerTargetService.id}
///   producerTargetService:
///     type: gcp:compute:ForwardingRule
///     name: producer_target_service
///     properties:
///       name: producer-forwarding-rule
///       region: us-central1
///       loadBalancingScheme: INTERNAL
///       backendService: ${producerServiceBackend.id}
///       allPorts: true
///       network: ${producerNet.name}
///       subnetwork: ${producerSubnet.name}
///   producerServiceBackend:
///     type: gcp:compute:RegionBackendService
///     name: producer_service_backend
///     properties:
///       name: producer-service-backend
///       region: us-central1
///       healthChecks: ${producerServiceHealthCheck.id}
///   producerServiceHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: producer_service_health_check
///     properties:
///       name: producer-service-health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
/// ```
/// ### Forwarding Rule Regional Steering
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = address::create(
///         "basic",
///         AddressArgs::builder().name("website-ip").region("us-central1").build_struct(),
///     );
///     let external = region_backend_service::create(
///         "external",
///         RegionBackendServiceArgs::builder()
///             .load_balancing_scheme("EXTERNAL")
///             .name("service-backend")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let externalForwardingRule = forwarding_rule::create(
///         "externalForwardingRule",
///         ForwardingRuleArgs::builder()
///             .backend_service("${external.selfLink}")
///             .ip_address("${basic.address}")
///             .load_balancing_scheme("EXTERNAL")
///             .name("external-forwarding-rule")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let steering = forwarding_rule::create(
///         "steering",
///         ForwardingRuleArgs::builder()
///             .backend_service("${external.selfLink}")
///             .ip_address("${basic.address}")
///             .load_balancing_scheme("EXTERNAL")
///             .name("steering-rule")
///             .region("us-central1")
///             .source_ip_ranges(vec!["34.121.88.0/24", "35.187.239.137",])
///             .build_struct(),
///     );
/// }
/// ```
/// ### Forwarding Rule Internallb Ipv6
///
///
/// ```yaml
/// resources:
///   # Forwarding rule for Internal Load Balancing
///   default:
///     type: gcp:compute:ForwardingRule
///     properties:
///       name: ilb-ipv6-forwarding-rule
///       region: us-central1
///       loadBalancingScheme: INTERNAL
///       backendService: ${backend.id}
///       allPorts: true
///       network: ${defaultNetwork.name}
///       subnetwork: ${defaultSubnetwork.name}
///       ipVersion: IPV6
///   backend:
///     type: gcp:compute:RegionBackendService
///     properties:
///       name: ilb-ipv6-backend
///       region: us-central1
///       healthChecks: ${hc.id}
///   hc:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: check-ilb-ipv6-backend
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: net-ipv6
///       autoCreateSubnetworks: false
///       enableUlaInternalIpv6: true
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: subnet-internal-ipv6
///       ipCidrRange: 10.0.0.0/16
///       region: us-central1
///       stackType: IPV4_IPV6
///       ipv6AccessType: INTERNAL
///       network: ${defaultNetwork.id}
/// ```
///
/// ## Import
///
/// ForwardingRule can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/forwardingRules/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, ForwardingRule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/forwardingRule:ForwardingRule default projects/{{project}}/regions/{{region}}/forwardingRules/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/forwardingRule:ForwardingRule default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/forwardingRule:ForwardingRule default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/forwardingRule:ForwardingRule default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod forwarding_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ForwardingRuleArgs {
        /// The `ports`, `portRange`, and `allPorts` fields are mutually exclusive.
        /// Only packets addressed to ports in the specified range will be forwarded
        /// to the backends configured with this forwarding rule.
        /// The `allPorts` field has the following limitations:
        /// * It requires that the forwarding rule `IPProtocol` be TCP, UDP, SCTP, or
        /// L3_DEFAULT.
        /// * It's applicable only to the following products: internal passthrough
        /// Network Load Balancers, backend service-based external passthrough Network
        /// Load Balancers, and internal and external protocol forwarding.
        /// * Set this field to true to allow packets addressed to any port or packets
        /// lacking destination port information (for example, UDP fragments after the
        /// first fragment) to be forwarded to the backends configured with this
        /// forwarding rule. The L3_DEFAULT protocol requires `allPorts` be set to
        /// true.
        #[builder(into, default)]
        pub all_ports: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// This field is used along with the `backend_service` field for
        /// internal load balancing or with the `target` field for internal
        /// TargetInstance.
        /// If the field is set to `TRUE`, clients can access ILB from all
        /// regions.
        /// Otherwise only allows access from clients in the same region as the
        /// internal load balancer.
        #[builder(into, default)]
        pub allow_global_access: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// This is used in PSC consumer ForwardingRule to control whether the PSC endpoint can be accessed from another region.
        #[builder(into, default)]
        pub allow_psc_global_access: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Identifies the backend service to which the forwarding rule sends traffic.
        /// Required for Internal TCP/UDP Load Balancing and Network Load Balancing;
        /// must be omitted for all other load balancer types.
        #[builder(into, default)]
        pub backend_service: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// IP address for which this forwarding rule accepts traffic. When a client
        /// sends traffic to this IP address, the forwarding rule directs the traffic
        /// to the referenced `target` or `backendService`.
        /// While creating a forwarding rule, specifying an `IPAddress` is
        /// required under the following circumstances:
        /// * When the `target` is set to `targetGrpcProxy` and
        /// `validateForProxyless` is set to `true`, the
        /// `IPAddress` should be set to `0.0.0.0`.
        /// * When the `target` is a Private Service Connect Google APIs
        /// bundle, you must specify an `IPAddress`.
        /// Otherwise, you can optionally specify an IP address that references an
        /// existing static (reserved) IP address resource. When omitted, Google Cloud
        /// assigns an ephemeral IP address.
        /// Use one of the following formats to specify an IP address while creating a
        /// forwarding rule:
        /// * IP address number, as in `100.1.2.3`
        /// * IPv6 address range, as in `2600:1234::/96`
        /// * Full resource URL, as in
        /// `https://www.googleapis.com/compute/v1/projects/project_id/regions/region/addresses/address-name`
        /// * Partial URL or by name, as in:
        /// * `projects/project_id/regions/region/addresses/address-name`
        /// * `regions/region/addresses/address-name`
        /// * `global/addresses/address-name`
        /// * `address-name`
        /// The forwarding rule's `target` or `backendService`,
        /// and in most cases, also the `loadBalancingScheme`, determine the
        /// type of IP address that you can use. For detailed information, see
        /// [IP address
        /// specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).
        /// When reading an `IPAddress`, the API always returns the IP
        /// address number.
        #[builder(into, default)]
        pub ip_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IP protocol to which this rule applies.
        /// For protocol forwarding, valid
        /// options are `TCP`, `UDP`, `ESP`,
        /// `AH`, `SCTP`, `ICMP` and
        /// `L3_DEFAULT`.
        /// The valid IP protocols are different for different load balancing products
        /// as described in [Load balancing
        /// features](https://cloud.google.com/load-balancing/docs/features#protocols_from_the_load_balancer_to_the_backends).
        /// A Forwarding Rule with protocol L3_DEFAULT can attach with target instance or
        /// backend service with UNSPECIFIED protocol.
        /// A forwarding rule with "L3_DEFAULT" IPProtocal cannot be attached to a backend service with TCP or UDP.
        /// Possible values are: `TCP`, `UDP`, `ESP`, `AH`, `SCTP`, `ICMP`, `L3_DEFAULT`.
        #[builder(into, default)]
        pub ip_protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IP address version that will be used by this forwarding rule.
        /// Valid options are IPV4 and IPV6.
        /// If not set, the IPv4 address will be used by default.
        /// Possible values are: `IPV4`, `IPV6`.
        #[builder(into, default)]
        pub ip_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether or not this load balancer can be used as a collector for
        /// packet mirroring. To prevent mirroring loops, instances behind this
        /// load balancer will not have their traffic mirrored even if a
        /// `PacketMirroring` rule applies to them.
        /// This can only be set to true for load balancers that have their
        /// `loadBalancingScheme` set to `INTERNAL`.
        #[builder(into, default)]
        pub is_mirroring_collector: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Labels to apply to this forwarding rule.  A list of key->value pairs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the forwarding rule type.
        /// For more information about forwarding rules, refer to
        /// [Forwarding rule concepts](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts).
        /// Default value is `EXTERNAL`.
        /// Possible values are: `EXTERNAL`, `EXTERNAL_MANAGED`, `INTERNAL`, `INTERNAL_MANAGED`.
        #[builder(into, default)]
        pub load_balancing_scheme: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the resource; provided by the client when the resource is created.
        /// The name must be 1-63 characters long, and comply with
        /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt).
        /// Specifically, the name must be 1-63 characters long and match the regular
        /// expression `a-z?` which means the first
        /// character must be a lowercase letter, and all following characters must
        /// be a dash, lowercase letter, or digit, except the last character, which
        /// cannot be a dash.
        /// For Private Service Connect forwarding rules that forward traffic to Google
        /// APIs, the forwarding rule name must be a 1-20 characters string with
        /// lowercase letters and numbers and must start with a letter.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This field is not used for external load balancing.
        /// For Internal TCP/UDP Load Balancing, this field identifies the network that
        /// the load balanced IP should belong to for this Forwarding Rule.
        /// If the subnetwork is specified, the network of the subnetwork will be used.
        /// If neither subnetwork nor this field is specified, the default network will
        /// be used.
        /// For Private Service Connect forwarding rules that forward traffic to Google
        /// APIs, a network must be provided.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This signifies the networking tier used for configuring
        /// this load balancer and can only take the following values:
        /// `PREMIUM`, `STANDARD`.
        /// For regional ForwardingRule, the valid values are `PREMIUM` and
        /// `STANDARD`. For GlobalForwardingRule, the valid value is
        /// `PREMIUM`.
        /// If this field is not specified, it is assumed to be `PREMIUM`.
        /// If `IPAddress` is specified, this value must be equal to the
        /// networkTier of the Address.
        /// Possible values are: `PREMIUM`, `STANDARD`.
        #[builder(into, default)]
        pub network_tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This is used in PSC consumer ForwardingRule to control whether it should try to auto-generate a DNS zone or not. Non-PSC forwarding rules do not use this field.
        #[builder(into, default)]
        pub no_automate_dns_zone: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The `ports`, `portRange`, and `allPorts` fields are mutually exclusive.
        /// Only packets addressed to ports in the specified range will be forwarded
        /// to the backends configured with this forwarding rule.
        /// The `portRange` field has the following limitations:
        /// * It requires that the forwarding rule `IPProtocol` be TCP, UDP, or SCTP,
        /// and
        /// * It's applicable only to the following products: external passthrough
        /// Network Load Balancers, internal and external proxy Network Load
        /// Balancers, internal and external Application Load Balancers, external
        /// protocol forwarding, and Classic VPN.
        /// * Some products have restrictions on what ports can be used. See
        /// [port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#port_specifications)
        /// for details.
        /// For external forwarding rules, two or more forwarding rules cannot use the
        /// same `[IPAddress, IPProtocol]` pair, and cannot have overlapping
        /// `portRange`s.
        /// For internal forwarding rules within the same VPC network, two or more
        /// forwarding rules cannot use the same `[IPAddress, IPProtocol]` pair, and
        /// cannot have overlapping `portRange`s.
        /// @pattern: \d+(?:-\d+)?
        #[builder(into, default)]
        pub port_range: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The `ports`, `portRange`, and `allPorts` fields are mutually exclusive.
        /// Only packets addressed to ports in the specified range will be forwarded
        /// to the backends configured with this forwarding rule.
        /// The `ports` field has the following limitations:
        /// * It requires that the forwarding rule `IPProtocol` be TCP, UDP, or SCTP,
        /// and
        /// * It's applicable only to the following products: internal passthrough
        /// Network Load Balancers, backend service-based external passthrough Network
        /// Load Balancers, and internal protocol forwarding.
        /// * You can specify a list of up to five ports by number, separated by
        /// commas. The ports can be contiguous or discontiguous.
        /// For external forwarding rules, two or more forwarding rules cannot use the
        /// same `[IPAddress, IPProtocol]` pair if they share at least one port
        /// number.
        /// For internal forwarding rules within the same VPC network, two or more
        /// forwarding rules cannot use the same `[IPAddress, IPProtocol]` pair if
        /// they share at least one port number.
        /// @pattern: \d+(?:-\d+)?
        #[builder(into, default)]
        pub ports: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub recreate_closed_psc: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A reference to the region where the regional forwarding rule resides.
        /// This field is not applicable to global forwarding rules.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Service Directory resources to register this forwarding rule with.
        /// Currently, only supports a single Service Directory resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub service_directory_registrations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::ForwardingRuleServiceDirectoryRegistrations,
            >,
        >,
        /// An optional prefix to the service name for this Forwarding Rule.
        /// If specified, will be the first label of the fully qualified service
        /// name.
        /// The label must be 1-63 characters long, and comply with RFC1035.
        /// Specifically, the label must be 1-63 characters long and match the
        /// regular expression `a-z?` which means the first
        /// character must be a lowercase letter, and all following characters
        /// must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        /// This field is only used for INTERNAL load balancing.
        #[builder(into, default)]
        pub service_label: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If not empty, this Forwarding Rule will only forward the traffic when the source IP address matches one of the IP addresses or CIDR ranges set here. Note that a Forwarding Rule can only have up to 64 source IP ranges, and this field can only be used with a regional Forwarding Rule whose scheme is EXTERNAL. Each sourceIpRange entry should be either an IP address (for example, 1.2.3.4) or a CIDR range (for example, 1.2.3.0/24).
        #[builder(into, default)]
        pub source_ip_ranges: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// This field identifies the subnetwork that the load balanced IP should
        /// belong to for this Forwarding Rule, used in internal load balancing and
        /// network load balancing with IPv6.
        /// If the network specified is in auto subnet mode, this field is optional.
        /// However, a subnetwork must be specified if the network is in custom subnet
        /// mode or when creating external forwarding rule with IPv6.
        #[builder(into, default)]
        pub subnetwork: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URL of the target resource to receive the matched traffic.  For
        /// regional forwarding rules, this target must be in the same region as the
        /// forwarding rule. For global forwarding rules, this target must be a global
        /// load balancing resource.
        /// The forwarded traffic must be of a type appropriate to the target object.
        /// *  For load balancers, see the "Target" column in [Port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).
        /// *  For Private Service Connect forwarding rules that forward traffic to Google APIs, provide the name of a supported Google API bundle:
        /// *  `vpc-sc` - [ APIs that support VPC Service Controls](https://cloud.google.com/vpc-service-controls/docs/supported-products).
        /// *  `all-apis` - [All supported Google APIs](https://cloud.google.com/vpc/docs/private-service-connect#supported-apis).
        /// For Private Service Connect forwarding rules that forward traffic to managed services, the target must be a service attachment.
        #[builder(into, default)]
        pub target: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ForwardingRuleResult {
        /// The `ports`, `portRange`, and `allPorts` fields are mutually exclusive.
        /// Only packets addressed to ports in the specified range will be forwarded
        /// to the backends configured with this forwarding rule.
        /// The `allPorts` field has the following limitations:
        /// * It requires that the forwarding rule `IPProtocol` be TCP, UDP, SCTP, or
        /// L3_DEFAULT.
        /// * It's applicable only to the following products: internal passthrough
        /// Network Load Balancers, backend service-based external passthrough Network
        /// Load Balancers, and internal and external protocol forwarding.
        /// * Set this field to true to allow packets addressed to any port or packets
        /// lacking destination port information (for example, UDP fragments after the
        /// first fragment) to be forwarded to the backends configured with this
        /// forwarding rule. The L3_DEFAULT protocol requires `allPorts` be set to
        /// true.
        pub all_ports: pulumi_gestalt_rust::Output<Option<bool>>,
        /// This field is used along with the `backend_service` field for
        /// internal load balancing or with the `target` field for internal
        /// TargetInstance.
        /// If the field is set to `TRUE`, clients can access ILB from all
        /// regions.
        /// Otherwise only allows access from clients in the same region as the
        /// internal load balancer.
        pub allow_global_access: pulumi_gestalt_rust::Output<Option<bool>>,
        /// This is used in PSC consumer ForwardingRule to control whether the PSC endpoint can be accessed from another region.
        pub allow_psc_global_access: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Identifies the backend service to which the forwarding rule sends traffic.
        /// Required for Internal TCP/UDP Load Balancing and Network Load Balancing;
        /// must be omitted for all other load balancer types.
        pub backend_service: pulumi_gestalt_rust::Output<Option<String>>,
        /// [Output Only] The URL for the corresponding base Forwarding Rule. By base Forwarding Rule, we mean the Forwarding Rule that has the same IP address, protocol, and port settings with the current Forwarding Rule, but without sourceIPRanges specified. Always empty if the current Forwarding Rule does not have sourceIPRanges specified.
        pub base_forwarding_rule: pulumi_gestalt_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The unique identifier number for the resource. This identifier is defined by the server.
        pub forwarding_rule_id: pulumi_gestalt_rust::Output<i32>,
        /// IP address for which this forwarding rule accepts traffic. When a client
        /// sends traffic to this IP address, the forwarding rule directs the traffic
        /// to the referenced `target` or `backendService`.
        /// While creating a forwarding rule, specifying an `IPAddress` is
        /// required under the following circumstances:
        /// * When the `target` is set to `targetGrpcProxy` and
        /// `validateForProxyless` is set to `true`, the
        /// `IPAddress` should be set to `0.0.0.0`.
        /// * When the `target` is a Private Service Connect Google APIs
        /// bundle, you must specify an `IPAddress`.
        /// Otherwise, you can optionally specify an IP address that references an
        /// existing static (reserved) IP address resource. When omitted, Google Cloud
        /// assigns an ephemeral IP address.
        /// Use one of the following formats to specify an IP address while creating a
        /// forwarding rule:
        /// * IP address number, as in `100.1.2.3`
        /// * IPv6 address range, as in `2600:1234::/96`
        /// * Full resource URL, as in
        /// `https://www.googleapis.com/compute/v1/projects/project_id/regions/region/addresses/address-name`
        /// * Partial URL or by name, as in:
        /// * `projects/project_id/regions/region/addresses/address-name`
        /// * `regions/region/addresses/address-name`
        /// * `global/addresses/address-name`
        /// * `address-name`
        /// The forwarding rule's `target` or `backendService`,
        /// and in most cases, also the `loadBalancingScheme`, determine the
        /// type of IP address that you can use. For detailed information, see
        /// [IP address
        /// specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).
        /// When reading an `IPAddress`, the API always returns the IP
        /// address number.
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// The IP protocol to which this rule applies.
        /// For protocol forwarding, valid
        /// options are `TCP`, `UDP`, `ESP`,
        /// `AH`, `SCTP`, `ICMP` and
        /// `L3_DEFAULT`.
        /// The valid IP protocols are different for different load balancing products
        /// as described in [Load balancing
        /// features](https://cloud.google.com/load-balancing/docs/features#protocols_from_the_load_balancer_to_the_backends).
        /// A Forwarding Rule with protocol L3_DEFAULT can attach with target instance or
        /// backend service with UNSPECIFIED protocol.
        /// A forwarding rule with "L3_DEFAULT" IPProtocal cannot be attached to a backend service with TCP or UDP.
        /// Possible values are: `TCP`, `UDP`, `ESP`, `AH`, `SCTP`, `ICMP`, `L3_DEFAULT`.
        pub ip_protocol: pulumi_gestalt_rust::Output<String>,
        /// The IP address version that will be used by this forwarding rule.
        /// Valid options are IPV4 and IPV6.
        /// If not set, the IPv4 address will be used by default.
        /// Possible values are: `IPV4`, `IPV6`.
        pub ip_version: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether or not this load balancer can be used as a collector for
        /// packet mirroring. To prevent mirroring loops, instances behind this
        /// load balancer will not have their traffic mirrored even if a
        /// `PacketMirroring` rule applies to them.
        /// This can only be set to true for load balancers that have their
        /// `loadBalancingScheme` set to `INTERNAL`.
        pub is_mirroring_collector: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The fingerprint used for optimistic locking of this resource.  Used
        /// internally during updates.
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Labels to apply to this forwarding rule.  A list of key->value pairs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the forwarding rule type.
        /// For more information about forwarding rules, refer to
        /// [Forwarding rule concepts](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts).
        /// Default value is `EXTERNAL`.
        /// Possible values are: `EXTERNAL`, `EXTERNAL_MANAGED`, `INTERNAL`, `INTERNAL_MANAGED`.
        pub load_balancing_scheme: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the resource; provided by the client when the resource is created.
        /// The name must be 1-63 characters long, and comply with
        /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt).
        /// Specifically, the name must be 1-63 characters long and match the regular
        /// expression `a-z?` which means the first
        /// character must be a lowercase letter, and all following characters must
        /// be a dash, lowercase letter, or digit, except the last character, which
        /// cannot be a dash.
        /// For Private Service Connect forwarding rules that forward traffic to Google
        /// APIs, the forwarding rule name must be a 1-20 characters string with
        /// lowercase letters and numbers and must start with a letter.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// This field is not used for external load balancing.
        /// For Internal TCP/UDP Load Balancing, this field identifies the network that
        /// the load balanced IP should belong to for this Forwarding Rule.
        /// If the subnetwork is specified, the network of the subnetwork will be used.
        /// If neither subnetwork nor this field is specified, the default network will
        /// be used.
        /// For Private Service Connect forwarding rules that forward traffic to Google
        /// APIs, a network must be provided.
        pub network: pulumi_gestalt_rust::Output<String>,
        /// This signifies the networking tier used for configuring
        /// this load balancer and can only take the following values:
        /// `PREMIUM`, `STANDARD`.
        /// For regional ForwardingRule, the valid values are `PREMIUM` and
        /// `STANDARD`. For GlobalForwardingRule, the valid value is
        /// `PREMIUM`.
        /// If this field is not specified, it is assumed to be `PREMIUM`.
        /// If `IPAddress` is specified, this value must be equal to the
        /// networkTier of the Address.
        /// Possible values are: `PREMIUM`, `STANDARD`.
        pub network_tier: pulumi_gestalt_rust::Output<String>,
        /// This is used in PSC consumer ForwardingRule to control whether it should try to auto-generate a DNS zone or not. Non-PSC forwarding rules do not use this field.
        pub no_automate_dns_zone: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The `ports`, `portRange`, and `allPorts` fields are mutually exclusive.
        /// Only packets addressed to ports in the specified range will be forwarded
        /// to the backends configured with this forwarding rule.
        /// The `portRange` field has the following limitations:
        /// * It requires that the forwarding rule `IPProtocol` be TCP, UDP, or SCTP,
        /// and
        /// * It's applicable only to the following products: external passthrough
        /// Network Load Balancers, internal and external proxy Network Load
        /// Balancers, internal and external Application Load Balancers, external
        /// protocol forwarding, and Classic VPN.
        /// * Some products have restrictions on what ports can be used. See
        /// [port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#port_specifications)
        /// for details.
        /// For external forwarding rules, two or more forwarding rules cannot use the
        /// same `[IPAddress, IPProtocol]` pair, and cannot have overlapping
        /// `portRange`s.
        /// For internal forwarding rules within the same VPC network, two or more
        /// forwarding rules cannot use the same `[IPAddress, IPProtocol]` pair, and
        /// cannot have overlapping `portRange`s.
        /// @pattern: \d+(?:-\d+)?
        pub port_range: pulumi_gestalt_rust::Output<String>,
        /// The `ports`, `portRange`, and `allPorts` fields are mutually exclusive.
        /// Only packets addressed to ports in the specified range will be forwarded
        /// to the backends configured with this forwarding rule.
        /// The `ports` field has the following limitations:
        /// * It requires that the forwarding rule `IPProtocol` be TCP, UDP, or SCTP,
        /// and
        /// * It's applicable only to the following products: internal passthrough
        /// Network Load Balancers, backend service-based external passthrough Network
        /// Load Balancers, and internal protocol forwarding.
        /// * You can specify a list of up to five ports by number, separated by
        /// commas. The ports can be contiguous or discontiguous.
        /// For external forwarding rules, two or more forwarding rules cannot use the
        /// same `[IPAddress, IPProtocol]` pair if they share at least one port
        /// number.
        /// For internal forwarding rules within the same VPC network, two or more
        /// forwarding rules cannot use the same `[IPAddress, IPProtocol]` pair if
        /// they share at least one port number.
        /// @pattern: \d+(?:-\d+)?
        pub ports: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The PSC connection id of the PSC Forwarding Rule.
        pub psc_connection_id: pulumi_gestalt_rust::Output<String>,
        /// The PSC connection status of the PSC Forwarding Rule. Possible values: `STATUS_UNSPECIFIED`, `PENDING`, `ACCEPTED`, `REJECTED`, `CLOSED`
        pub psc_connection_status: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub recreate_closed_psc: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A reference to the region where the regional forwarding rule resides.
        /// This field is not applicable to global forwarding rules.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Service Directory resources to register this forwarding rule with.
        /// Currently, only supports a single Service Directory resource.
        /// Structure is documented below.
        pub service_directory_registrations: pulumi_gestalt_rust::Output<
            super::super::types::compute::ForwardingRuleServiceDirectoryRegistrations,
        >,
        /// An optional prefix to the service name for this Forwarding Rule.
        /// If specified, will be the first label of the fully qualified service
        /// name.
        /// The label must be 1-63 characters long, and comply with RFC1035.
        /// Specifically, the label must be 1-63 characters long and match the
        /// regular expression `a-z?` which means the first
        /// character must be a lowercase letter, and all following characters
        /// must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        /// This field is only used for INTERNAL load balancing.
        pub service_label: pulumi_gestalt_rust::Output<Option<String>>,
        /// The internal fully qualified service name for this Forwarding Rule.
        /// This field is only used for INTERNAL load balancing.
        pub service_name: pulumi_gestalt_rust::Output<String>,
        /// If not empty, this Forwarding Rule will only forward the traffic when the source IP address matches one of the IP addresses or CIDR ranges set here. Note that a Forwarding Rule can only have up to 64 source IP ranges, and this field can only be used with a regional Forwarding Rule whose scheme is EXTERNAL. Each sourceIpRange entry should be either an IP address (for example, 1.2.3.4) or a CIDR range (for example, 1.2.3.0/24).
        pub source_ip_ranges: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// This field identifies the subnetwork that the load balanced IP should
        /// belong to for this Forwarding Rule, used in internal load balancing and
        /// network load balancing with IPv6.
        /// If the network specified is in auto subnet mode, this field is optional.
        /// However, a subnetwork must be specified if the network is in custom subnet
        /// mode or when creating external forwarding rule with IPv6.
        pub subnetwork: pulumi_gestalt_rust::Output<String>,
        /// The URL of the target resource to receive the matched traffic.  For
        /// regional forwarding rules, this target must be in the same region as the
        /// forwarding rule. For global forwarding rules, this target must be a global
        /// load balancing resource.
        /// The forwarded traffic must be of a type appropriate to the target object.
        /// *  For load balancers, see the "Target" column in [Port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).
        /// *  For Private Service Connect forwarding rules that forward traffic to Google APIs, provide the name of a supported Google API bundle:
        /// *  `vpc-sc` - [ APIs that support VPC Service Controls](https://cloud.google.com/vpc-service-controls/docs/supported-products).
        /// *  `all-apis` - [All supported Google APIs](https://cloud.google.com/vpc/docs/private-service-connect#supported-apis).
        /// For Private Service Connect forwarding rules that forward traffic to managed services, the target must be a service attachment.
        pub target: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ForwardingRuleArgs,
    ) -> ForwardingRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let all_ports_binding_1 = args.all_ports.get_output(context);
        let all_ports_binding = all_ports_binding_1.get_inner();
        let allow_global_access_binding_1 = args.allow_global_access.get_output(context);
        let allow_global_access_binding = allow_global_access_binding_1.get_inner();
        let allow_psc_global_access_binding_1 = args
            .allow_psc_global_access
            .get_output(context);
        let allow_psc_global_access_binding = allow_psc_global_access_binding_1
            .get_inner();
        let backend_service_binding_1 = args.backend_service.get_output(context);
        let backend_service_binding = backend_service_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let ip_address_binding_1 = args.ip_address.get_output(context);
        let ip_address_binding = ip_address_binding_1.get_inner();
        let ip_protocol_binding_1 = args.ip_protocol.get_output(context);
        let ip_protocol_binding = ip_protocol_binding_1.get_inner();
        let ip_version_binding_1 = args.ip_version.get_output(context);
        let ip_version_binding = ip_version_binding_1.get_inner();
        let is_mirroring_collector_binding_1 = args
            .is_mirroring_collector
            .get_output(context);
        let is_mirroring_collector_binding = is_mirroring_collector_binding_1
            .get_inner();
        let labels_binding_1 = args.labels.get_output(context);
        let labels_binding = labels_binding_1.get_inner();
        let load_balancing_scheme_binding_1 = args
            .load_balancing_scheme
            .get_output(context);
        let load_balancing_scheme_binding = load_balancing_scheme_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let network_binding_1 = args.network.get_output(context);
        let network_binding = network_binding_1.get_inner();
        let network_tier_binding_1 = args.network_tier.get_output(context);
        let network_tier_binding = network_tier_binding_1.get_inner();
        let no_automate_dns_zone_binding_1 = args
            .no_automate_dns_zone
            .get_output(context);
        let no_automate_dns_zone_binding = no_automate_dns_zone_binding_1.get_inner();
        let port_range_binding_1 = args.port_range.get_output(context);
        let port_range_binding = port_range_binding_1.get_inner();
        let ports_binding_1 = args.ports.get_output(context);
        let ports_binding = ports_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let recreate_closed_psc_binding_1 = args.recreate_closed_psc.get_output(context);
        let recreate_closed_psc_binding = recreate_closed_psc_binding_1.get_inner();
        let region_binding_1 = args.region.get_output(context);
        let region_binding = region_binding_1.get_inner();
        let service_directory_registrations_binding_1 = args
            .service_directory_registrations
            .get_output(context);
        let service_directory_registrations_binding = service_directory_registrations_binding_1
            .get_inner();
        let service_label_binding_1 = args.service_label.get_output(context);
        let service_label_binding = service_label_binding_1.get_inner();
        let source_ip_ranges_binding_1 = args.source_ip_ranges.get_output(context);
        let source_ip_ranges_binding = source_ip_ranges_binding_1.get_inner();
        let subnetwork_binding_1 = args.subnetwork.get_output(context);
        let subnetwork_binding = subnetwork_binding_1.get_inner();
        let target_binding_1 = args.target.get_output(context);
        let target_binding = target_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/forwardingRule:ForwardingRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allPorts".into(),
                    value: &all_ports_binding,
                },
                register_interface::ObjectField {
                    name: "allowGlobalAccess".into(),
                    value: &allow_global_access_binding,
                },
                register_interface::ObjectField {
                    name: "allowPscGlobalAccess".into(),
                    value: &allow_psc_global_access_binding,
                },
                register_interface::ObjectField {
                    name: "backendService".into(),
                    value: &backend_service_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddress".into(),
                    value: &ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "ipProtocol".into(),
                    value: &ip_protocol_binding,
                },
                register_interface::ObjectField {
                    name: "ipVersion".into(),
                    value: &ip_version_binding,
                },
                register_interface::ObjectField {
                    name: "isMirroringCollector".into(),
                    value: &is_mirroring_collector_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancingScheme".into(),
                    value: &load_balancing_scheme_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "networkTier".into(),
                    value: &network_tier_binding,
                },
                register_interface::ObjectField {
                    name: "noAutomateDnsZone".into(),
                    value: &no_automate_dns_zone_binding,
                },
                register_interface::ObjectField {
                    name: "portRange".into(),
                    value: &port_range_binding,
                },
                register_interface::ObjectField {
                    name: "ports".into(),
                    value: &ports_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "recreateClosedPsc".into(),
                    value: &recreate_closed_psc_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "serviceDirectoryRegistrations".into(),
                    value: &service_directory_registrations_binding,
                },
                register_interface::ObjectField {
                    name: "serviceLabel".into(),
                    value: &service_label_binding,
                },
                register_interface::ObjectField {
                    name: "sourceIpRanges".into(),
                    value: &source_ip_ranges_binding,
                },
                register_interface::ObjectField {
                    name: "subnetwork".into(),
                    value: &subnetwork_binding,
                },
                register_interface::ObjectField {
                    name: "target".into(),
                    value: &target_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ForwardingRuleResult {
            all_ports: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allPorts"),
            ),
            allow_global_access: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowGlobalAccess"),
            ),
            allow_psc_global_access: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowPscGlobalAccess"),
            ),
            backend_service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backendService"),
            ),
            base_forwarding_rule: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("baseForwardingRule"),
            ),
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            forwarding_rule_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forwardingRuleId"),
            ),
            ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddress"),
            ),
            ip_protocol: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipProtocol"),
            ),
            ip_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipVersion"),
            ),
            is_mirroring_collector: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isMirroringCollector"),
            ),
            label_fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labelFingerprint"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            load_balancing_scheme: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loadBalancingScheme"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            network_tier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkTier"),
            ),
            no_automate_dns_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("noAutomateDnsZone"),
            ),
            port_range: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("portRange"),
            ),
            ports: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ports")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            psc_connection_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pscConnectionId"),
            ),
            psc_connection_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pscConnectionStatus"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            recreate_closed_psc: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recreateClosedPsc"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            service_directory_registrations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceDirectoryRegistrations"),
            ),
            service_label: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceLabel"),
            ),
            service_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceName"),
            ),
            source_ip_ranges: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceIpRanges"),
            ),
            subnetwork: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetwork"),
            ),
            target: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("target"),
            ),
        }
    }
}
