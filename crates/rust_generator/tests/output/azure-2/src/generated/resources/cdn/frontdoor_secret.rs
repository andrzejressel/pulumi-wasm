/// Manages a Front Door (standard/premium) Secret.
///
/// ```New-AzADServicePrincipal -ApplicationId "00000000-0000-0000-0000-000000000000"```
///
/// | Object ID                                | Key Permissions | Secret Permissions   | Certificate Permissions                       |
/// |:-----------------------------------------|:---------------:|:--------------------:|:---------------------------------------------:|
/// | `Microsoft.Azure.Cdn` Object ID          | -               | **Get**              | -                                             |
/// | Your Personal AAD Object ID              | -               | **Get** and **List** | **Get**, **List**, **Purge** and **Recover**  |
/// | Terraform Service Principal              | -               | **Get**              | **Get**, **Import**, **Delete** and **Purge** |
///
/// ->**NOTE:** You only need to add the `Access Policy` for your personal AAD Object ID if you are planning to view the `secrets` via the Azure Portal.
///
/// ## Example Usage
///
/// <!--Start PulumiCodeChooser -->
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as azure from "@pulumi/azure";
/// import * as azuread from "@pulumi/azuread";
/// import * as std from "@pulumi/std";
///
/// const current = azure.core.getClientConfig({});
/// const frontdoor = azuread.getServicePrincipal({
///     displayName: "Microsoft.Azure.Cdn",
/// });
/// const example = new azure.core.ResourceGroup("example", {
///     name: "example-cdn-frontdoor",
///     location: "West Europe",
/// });
/// const exampleKeyVault = new azure.keyvault.KeyVault("example", {
///     name: "example-keyvault",
///     location: example.location,
///     resourceGroupName: example.name,
///     tenantId: current.then(current => current.tenantId),
///     skuName: "premium",
///     softDeleteRetentionDays: 7,
///     networkAcls: {
///         defaultAction: "Deny",
///         bypass: "AzureServices",
///         ipRules: ["10.0.0.0/24"],
///     },
///     accessPolicies: [
///         {
///             tenantId: current.then(current => current.tenantId),
///             objectId: frontdoor.then(frontdoor => frontdoor.objectId),
///             secretPermissions: ["Get"],
///         },
///         {
///             tenantId: current.then(current => current.tenantId),
///             objectId: current.then(current => current.objectId),
///             certificatePermissions: [
///                 "Get",
///                 "Import",
///                 "Delete",
///                 "Purge",
///             ],
///             secretPermissions: ["Get"],
///         },
///     ],
/// });
/// const exampleCertificate = new azure.keyvault.Certificate("example", {
///     name: "example-cert",
///     keyVaultId: exampleKeyVault.id,
///     certificate: {
///         contents: std.filebase64({
///             input: "my-certificate.pfx",
///         }).then(invoke => invoke.result),
///     },
/// });
/// const exampleFrontdoorProfile = new azure.cdn.FrontdoorProfile("example", {
///     name: "example-cdn-profile",
///     resourceGroupName: example.name,
///     skuName: "Standard_AzureFrontDoor",
/// });
/// const exampleFrontdoorSecret = new azure.cdn.FrontdoorSecret("example", {
///     name: "example-customer-managed-secret",
///     cdnFrontdoorProfileId: exampleFrontdoorProfile.id,
///     secret: {
///         customerCertificates: [{
///             keyVaultCertificateId: exampleCertificate.id,
///         }],
///     },
/// });
/// ```
/// ```python
/// import pulumi
/// import pulumi_azure as azure
/// import pulumi_azuread as azuread
/// import pulumi_std as std
///
/// current = azure.core.get_client_config()
/// frontdoor = azuread.get_service_principal(display_name="Microsoft.Azure.Cdn")
/// example = azure.core.ResourceGroup("example",
///     name="example-cdn-frontdoor",
///     location="West Europe")
/// example_key_vault = azure.keyvault.KeyVault("example",
///     name="example-keyvault",
///     location=example.location,
///     resource_group_name=example.name,
///     tenant_id=current.tenant_id,
///     sku_name="premium",
///     soft_delete_retention_days=7,
///     network_acls={
///         "default_action": "Deny",
///         "bypass": "AzureServices",
///         "ip_rules": ["10.0.0.0/24"],
///     },
///     access_policies=[
///         {
///             "tenant_id": current.tenant_id,
///             "object_id": frontdoor.object_id,
///             "secret_permissions": ["Get"],
///         },
///         {
///             "tenant_id": current.tenant_id,
///             "object_id": current.object_id,
///             "certificate_permissions": [
///                 "Get",
///                 "Import",
///                 "Delete",
///                 "Purge",
///             ],
///             "secret_permissions": ["Get"],
///         },
///     ])
/// example_certificate = azure.keyvault.Certificate("example",
///     name="example-cert",
///     key_vault_id=example_key_vault.id,
///     certificate={
///         "contents": std.filebase64(input="my-certificate.pfx").result,
///     })
/// example_frontdoor_profile = azure.cdn.FrontdoorProfile("example",
///     name="example-cdn-profile",
///     resource_group_name=example.name,
///     sku_name="Standard_AzureFrontDoor")
/// example_frontdoor_secret = azure.cdn.FrontdoorSecret("example",
///     name="example-customer-managed-secret",
///     cdn_frontdoor_profile_id=example_frontdoor_profile.id,
///     secret={
///         "customer_certificates": [{
///             "key_vault_certificate_id": example_certificate.id,
///         }],
///     })
/// ```
/// ```csharp
/// using System.Collections.Generic;
/// using System.Linq;
/// using Pulumi;
/// using Azure = Pulumi.Azure;
/// using AzureAD = Pulumi.AzureAD;
/// using Std = Pulumi.Std;
///
/// return await Deployment.RunAsync(() =>
/// {
///     var current = Azure.Core.GetClientConfig.Invoke();
///
///     var frontdoor = AzureAD.GetServicePrincipal.Invoke(new()
///     {
///         DisplayName = "Microsoft.Azure.Cdn",
///     });
///
///     var example = new Azure.Core.ResourceGroup("example", new()
///     {
///         Name = "example-cdn-frontdoor",
///         Location = "West Europe",
///     });
///
///     var exampleKeyVault = new Azure.KeyVault.KeyVault("example", new()
///     {
///         Name = "example-keyvault",
///         Location = example.Location,
///         ResourceGroupName = example.Name,
///         TenantId = current.Apply(getClientConfigResult => getClientConfigResult.TenantId),
///         SkuName = "premium",
///         SoftDeleteRetentionDays = 7,
///         NetworkAcls = new Azure.KeyVault.Inputs.KeyVaultNetworkAclsArgs
///         {
///             DefaultAction = "Deny",
///             Bypass = "AzureServices",
///             IpRules = new[]
///             {
///                 "10.0.0.0/24",
///             },
///         },
///         AccessPolicies = new[]
///         {
///             new Azure.KeyVault.Inputs.KeyVaultAccessPolicyArgs
///             {
///                 TenantId = current.Apply(getClientConfigResult => getClientConfigResult.TenantId),
///                 ObjectId = frontdoor.Apply(getServicePrincipalResult => getServicePrincipalResult.ObjectId),
///                 SecretPermissions = new[]
///                 {
///                     "Get",
///                 },
///             },
///             new Azure.KeyVault.Inputs.KeyVaultAccessPolicyArgs
///             {
///                 TenantId = current.Apply(getClientConfigResult => getClientConfigResult.TenantId),
///                 ObjectId = current.Apply(getClientConfigResult => getClientConfigResult.ObjectId),
///                 CertificatePermissions = new[]
///                 {
///                     "Get",
///                     "Import",
///                     "Delete",
///                     "Purge",
///                 },
///                 SecretPermissions = new[]
///                 {
///                     "Get",
///                 },
///             },
///         },
///     });
///
///     var exampleCertificate = new Azure.KeyVault.Certificate("example", new()
///     {
///         Name = "example-cert",
///         KeyVaultId = exampleKeyVault.Id,
///         KeyVaultCertificate = new Azure.KeyVault.Inputs.CertificateCertificateArgs
///         {
///             Contents = Std.Filebase64.Invoke(new()
///             {
///                 Input = "my-certificate.pfx",
///             }).Apply(invoke => invoke.Result),
///         },
///     });
///
///     var exampleFrontdoorProfile = new Azure.Cdn.FrontdoorProfile("example", new()
///     {
///         Name = "example-cdn-profile",
///         ResourceGroupName = example.Name,
///         SkuName = "Standard_AzureFrontDoor",
///     });
///
///     var exampleFrontdoorSecret = new Azure.Cdn.FrontdoorSecret("example", new()
///     {
///         Name = "example-customer-managed-secret",
///         CdnFrontdoorProfileId = exampleFrontdoorProfile.Id,
///         Secret = new Azure.Cdn.Inputs.FrontdoorSecretSecretArgs
///         {
///             CustomerCertificates = new[]
///             {
///                 new Azure.Cdn.Inputs.FrontdoorSecretSecretCustomerCertificateArgs
///                 {
///                     KeyVaultCertificateId = exampleCertificate.Id,
///                 },
///             },
///         },
///     });
///
/// });
/// ```
/// ```go
/// package main
///
/// import (
/// 	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/cdn"
/// 	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
/// 	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/keyvault"
/// 	"github.com/pulumi/pulumi-azuread/sdk/v5/go/azuread"
/// 	"github.com/pulumi/pulumi-std/sdk/go/std"
/// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
/// )
///
/// func main() {
/// 	pulumi.Run(func(ctx *pulumi.Context) error {
/// 		current, err := core.GetClientConfig(ctx, map[string]interface{}{}, nil)
/// 		if err != nil {
/// 			return err
/// 		}
/// 		frontdoor, err := azuread.LookupServicePrincipal(ctx, &azuread.LookupServicePrincipalArgs{
/// 			DisplayName: pulumi.StringRef("Microsoft.Azure.Cdn"),
/// 		}, nil)
/// 		if err != nil {
/// 			return err
/// 		}
/// 		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
/// 			Name:     pulumi.String("example-cdn-frontdoor"),
/// 			Location: pulumi.String("West Europe"),
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		exampleKeyVault, err := keyvault.NewKeyVault(ctx, "example", &keyvault.KeyVaultArgs{
/// 			Name:                    pulumi.String("example-keyvault"),
/// 			Location:                example.Location,
/// 			ResourceGroupName:       example.Name,
/// 			TenantId:                pulumi.String(current.TenantId),
/// 			SkuName:                 pulumi.String("premium"),
/// 			SoftDeleteRetentionDays: pulumi.Int(7),
/// 			NetworkAcls: &keyvault.KeyVaultNetworkAclsArgs{
/// 				DefaultAction: pulumi.String("Deny"),
/// 				Bypass:        pulumi.String("AzureServices"),
/// 				IpRules: pulumi.StringArray{
/// 					pulumi.String("10.0.0.0/24"),
/// 				},
/// 			},
/// 			AccessPolicies: keyvault.KeyVaultAccessPolicyArray{
/// 				&keyvault.KeyVaultAccessPolicyArgs{
/// 					TenantId: pulumi.String(current.TenantId),
/// 					ObjectId: pulumi.String(frontdoor.ObjectId),
/// 					SecretPermissions: pulumi.StringArray{
/// 						pulumi.String("Get"),
/// 					},
/// 				},
/// 				&keyvault.KeyVaultAccessPolicyArgs{
/// 					TenantId: pulumi.String(current.TenantId),
/// 					ObjectId: pulumi.String(current.ObjectId),
/// 					CertificatePermissions: pulumi.StringArray{
/// 						pulumi.String("Get"),
/// 						pulumi.String("Import"),
/// 						pulumi.String("Delete"),
/// 						pulumi.String("Purge"),
/// 					},
/// 					SecretPermissions: pulumi.StringArray{
/// 						pulumi.String("Get"),
/// 					},
/// 				},
/// 			},
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		invokeFilebase64, err := std.Filebase64(ctx, &std.Filebase64Args{
/// 			Input: "my-certificate.pfx",
/// 		}, nil)
/// 		if err != nil {
/// 			return err
/// 		}
/// 		exampleCertificate, err := keyvault.NewCertificate(ctx, "example", &keyvault.CertificateArgs{
/// 			Name:       pulumi.String("example-cert"),
/// 			KeyVaultId: exampleKeyVault.ID(),
/// 			Certificate: &keyvault.CertificateCertificateArgs{
/// 				Contents: pulumi.String(invokeFilebase64.Result),
/// 			},
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		exampleFrontdoorProfile, err := cdn.NewFrontdoorProfile(ctx, "example", &cdn.FrontdoorProfileArgs{
/// 			Name:              pulumi.String("example-cdn-profile"),
/// 			ResourceGroupName: example.Name,
/// 			SkuName:           pulumi.String("Standard_AzureFrontDoor"),
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		_, err = cdn.NewFrontdoorSecret(ctx, "example", &cdn.FrontdoorSecretArgs{
/// 			Name:                  pulumi.String("example-customer-managed-secret"),
/// 			CdnFrontdoorProfileId: exampleFrontdoorProfile.ID(),
/// 			Secret: &cdn.FrontdoorSecretSecretArgs{
/// 				CustomerCertificates: cdn.FrontdoorSecretSecretCustomerCertificateArray{
/// 					&cdn.FrontdoorSecretSecretCustomerCertificateArgs{
/// 						KeyVaultCertificateId: exampleCertificate.ID(),
/// 					},
/// 				},
/// 			},
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		return nil
/// 	})
/// }
/// ```
/// ```java
/// package generated_program;
///
/// import com.pulumi.Context;
/// import com.pulumi.Pulumi;
/// import com.pulumi.core.Output;
/// import com.pulumi.azure.core.CoreFunctions;
/// import com.pulumi.azuread.AzureadFunctions;
/// import com.pulumi.azuread.inputs.GetServicePrincipalArgs;
/// import com.pulumi.azure.core.ResourceGroup;
/// import com.pulumi.azure.core.ResourceGroupArgs;
/// import com.pulumi.azure.keyvault.KeyVault;
/// import com.pulumi.azure.keyvault.KeyVaultArgs;
/// import com.pulumi.azure.keyvault.inputs.KeyVaultNetworkAclsArgs;
/// import com.pulumi.azure.keyvault.inputs.KeyVaultAccessPolicyArgs;
/// import com.pulumi.azure.keyvault.Certificate;
/// import com.pulumi.azure.keyvault.CertificateArgs;
/// import com.pulumi.azure.keyvault.inputs.CertificateCertificateArgs;
/// import com.pulumi.azure.cdn.FrontdoorProfile;
/// import com.pulumi.azure.cdn.FrontdoorProfileArgs;
/// import com.pulumi.azure.cdn.FrontdoorSecret;
/// import com.pulumi.azure.cdn.FrontdoorSecretArgs;
/// import com.pulumi.azure.cdn.inputs.FrontdoorSecretSecretArgs;
/// import java.util.List;
/// import java.util.ArrayList;
/// import java.util.Map;
/// import java.io.File;
/// import java.nio.file.Files;
/// import java.nio.file.Paths;
///
/// public class App {
///     public static void main(String[] args) {
///         Pulumi.run(App::stack);
///     }
///
///     public static void stack(Context ctx) {
///         final var current = CoreFunctions.getClientConfig();
///
///         final var frontdoor = AzureadFunctions.getServicePrincipal(GetServicePrincipalArgs.builder()
///             .displayName("Microsoft.Azure.Cdn")
///             .build());
///
///         var example = new ResourceGroup("example", ResourceGroupArgs.builder()
///             .name("example-cdn-frontdoor")
///             .location("West Europe")
///             .build());
///
///         var exampleKeyVault = new KeyVault("exampleKeyVault", KeyVaultArgs.builder()
///             .name("example-keyvault")
///             .location(example.location())
///             .resourceGroupName(example.name())
///             .tenantId(current.applyValue(getClientConfigResult -> getClientConfigResult.tenantId()))
///             .skuName("premium")
///             .softDeleteRetentionDays(7)
///             .networkAcls(KeyVaultNetworkAclsArgs.builder()
///                 .defaultAction("Deny")
///                 .bypass("AzureServices")
///                 .ipRules("10.0.0.0/24")
///                 .build())
///             .accessPolicies(
///                 KeyVaultAccessPolicyArgs.builder()
///                     .tenantId(current.applyValue(getClientConfigResult -> getClientConfigResult.tenantId()))
///                     .objectId(frontdoor.applyValue(getServicePrincipalResult -> getServicePrincipalResult.objectId()))
///                     .secretPermissions("Get")
///                     .build(),
///                 KeyVaultAccessPolicyArgs.builder()
///                     .tenantId(current.applyValue(getClientConfigResult -> getClientConfigResult.tenantId()))
///                     .objectId(current.applyValue(getClientConfigResult -> getClientConfigResult.objectId()))
///                     .certificatePermissions(
///                         "Get",
///                         "Import",
///                         "Delete",
///                         "Purge")
///                     .secretPermissions("Get")
///                     .build())
///             .build());
///
///         var exampleCertificate = new Certificate("exampleCertificate", CertificateArgs.builder()
///             .name("example-cert")
///             .keyVaultId(exampleKeyVault.id())
///             .certificate(CertificateCertificateArgs.builder()
///                 .contents(StdFunctions.filebase64(Filebase64Args.builder()
///                     .input("my-certificate.pfx")
///                     .build()).result())
///                 .build())
///             .build());
///
///         var exampleFrontdoorProfile = new FrontdoorProfile("exampleFrontdoorProfile", FrontdoorProfileArgs.builder()
///             .name("example-cdn-profile")
///             .resourceGroupName(example.name())
///             .skuName("Standard_AzureFrontDoor")
///             .build());
///
///         var exampleFrontdoorSecret = new FrontdoorSecret("exampleFrontdoorSecret", FrontdoorSecretArgs.builder()
///             .name("example-customer-managed-secret")
///             .cdnFrontdoorProfileId(exampleFrontdoorProfile.id())
///             .secret(FrontdoorSecretSecretArgs.builder()
///                 .customerCertificates(FrontdoorSecretSecretCustomerCertificateArgs.builder()
///                     .keyVaultCertificateId(exampleCertificate.id())
///                     .build())
///                 .build())
///             .build());
///
///     }
/// }
/// ```
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-cdn-frontdoor
///       location: West Europe
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: example-keyvault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///       softDeleteRetentionDays: 7
///       networkAcls:
///         defaultAction: Deny
///         bypass: AzureServices
///         ipRules:
///           - 10.0.0.0/24
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${frontdoor.objectId}
///           secretPermissions:
///             - Get
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           certificatePermissions:
///             - Get
///             - Import
///             - Delete
///             - Purge
///           secretPermissions:
///             - Get
///   exampleCertificate:
///     type: azure:keyvault:Certificate
///     name: example
///     properties:
///       name: example-cert
///       keyVaultId: ${exampleKeyVault.id}
///       certificate:
///         contents:
///           fn::invoke:
///             function: std:filebase64
///             arguments:
///               input: my-certificate.pfx
///             return: result
///   exampleFrontdoorProfile:
///     type: azure:cdn:FrontdoorProfile
///     name: example
///     properties:
///       name: example-cdn-profile
///       resourceGroupName: ${example.name}
///       skuName: Standard_AzureFrontDoor
///   exampleFrontdoorSecret:
///     type: azure:cdn:FrontdoorSecret
///     name: example
///     properties:
///       name: example-customer-managed-secret
///       cdnFrontdoorProfileId: ${exampleFrontdoorProfile.id}
///       secret:
///         customerCertificates:
///           - keyVaultCertificateId: ${exampleCertificate.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   frontdoor:
///     fn::invoke:
///       function: azuread:getServicePrincipal
///       arguments:
///         displayName: Microsoft.Azure.Cdn
/// ```
/// <!--End PulumiCodeChooser -->
///
/// ## Import
///
/// Front Door Secrets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cdn/frontdoorSecret:FrontdoorSecret example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Cdn/profiles/profile1/secrets/secrets1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod frontdoor_secret {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrontdoorSecretArgs {
        /// The Resource ID of the Front Door Profile. Changing this forces a new Front Door Secret to be created.
        #[builder(into)]
        pub cdn_frontdoor_profile_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Front Door Secret. Possible values must start with a letter or a number, only contain letters, numbers and hyphens and have a length of between 2 and 260 characters. Changing this forces a new Front Door Secret to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `secret` block as defined below. Changing this forces a new Front Door Secret to be created.
        #[builder(into)]
        pub secret: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cdn::FrontdoorSecretSecret,
        >,
    }
    #[allow(dead_code)]
    pub struct FrontdoorSecretResult {
        /// The Resource ID of the Front Door Profile. Changing this forces a new Front Door Secret to be created.
        pub cdn_frontdoor_profile_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Front Door Profile containing this Front Door Secret.
        pub cdn_frontdoor_profile_name: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Front Door Secret. Possible values must start with a letter or a number, only contain letters, numbers and hyphens and have a length of between 2 and 260 characters. Changing this forces a new Front Door Secret to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `secret` block as defined below. Changing this forces a new Front Door Secret to be created.
        pub secret: pulumi_gestalt_rust::Output<
            super::super::types::cdn::FrontdoorSecretSecret,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FrontdoorSecretArgs,
    ) -> FrontdoorSecretResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cdn_frontdoor_profile_id_binding = args
            .cdn_frontdoor_profile_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let secret_binding = args.secret.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cdn/frontdoorSecret:FrontdoorSecret".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cdnFrontdoorProfileId".into(),
                    value: cdn_frontdoor_profile_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secret".into(),
                    value: secret_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FrontdoorSecretResult {
            cdn_frontdoor_profile_id: o.get_field("cdnFrontdoorProfileId"),
            cdn_frontdoor_profile_name: o.get_field("cdnFrontdoorProfileName"),
            name: o.get_field("name"),
            secret: o.get_field("secret"),
        }
    }
}
