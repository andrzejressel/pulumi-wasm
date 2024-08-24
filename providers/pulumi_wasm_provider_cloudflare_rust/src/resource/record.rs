//! Provides a Cloudflare record resource.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! // Add a record to the domain
//! const example = new cloudflare.Record("example", {
//!     zoneId: _var.cloudflare_zone_id,
//!     name: "example",
//!     value: "192.0.2.1",
//!     type: "A",
//!     ttl: 3600,
//! });
//! // Add a record requiring a data map
//! const _sipTls = new cloudflare.Record("_sipTls", {
//!     zoneId: _var.cloudflare_zone_id,
//!     name: "_sip._tls",
//!     type: "SRV",
//!     data: {
//!         service: "_sip",
//!         proto: "_tls",
//!         name: "example-srv",
//!         priority: 0,
//!         weight: 0,
//!         port: 443,
//!         target: "example.com",
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! # Add a record to the domain
//! example = cloudflare.Record("example",
//!     zone_id=var["cloudflare_zone_id"],
//!     name="example",
//!     value="192.0.2.1",
//!     type="A",
//!     ttl=3600)
//! # Add a record requiring a data map
//! _sip_tls = cloudflare.Record("_sipTls",
//!     zone_id=var["cloudflare_zone_id"],
//!     name="_sip._tls",
//!     type="SRV",
//!     data=cloudflare.RecordDataArgs(
//!         service="_sip",
//!         proto="_tls",
//!         name="example-srv",
//!         priority=0,
//!         weight=0,
//!         port=443,
//!         target="example.com",
//!     ))
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Cloudflare = Pulumi.Cloudflare;
//!
//! return await Deployment.RunAsync(() =>
//! {
//!     // Add a record to the domain
//!     var example = new Cloudflare.Record("example", new()
//!     {
//!         ZoneId = @var.Cloudflare_zone_id,
//!         Name = "example",
//!         Value = "192.0.2.1",
//!         Type = "A",
//!         Ttl = 3600,
//!     });
//!
//!     // Add a record requiring a data map
//!     var _sipTls = new Cloudflare.Record("_sipTls", new()
//!     {
//!         ZoneId = @var.Cloudflare_zone_id,
//!         Name = "_sip._tls",
//!         Type = "SRV",
//!         Data = new Cloudflare.Inputs.RecordDataArgs
//!         {
//!             Service = "_sip",
//!             Proto = "_tls",
//!             Name = "example-srv",
//!             Priority = 0,
//!             Weight = 0,
//!             Port = 443,
//!             Target = "example.com",
//!         },
//!     });
//!
//! });
//! ```
//! ### Go
//! ```go
//! package main
//!
//! import (
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//!
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		// Add a record to the domain
//! 		_, err := cloudflare.NewRecord(ctx, "example", &cloudflare.RecordArgs{
//! 			ZoneId: pulumi.Any(_var.Cloudflare_zone_id),
//! 			Name:   pulumi.String("example"),
//! 			Value:  pulumi.String("192.0.2.1"),
//! 			Type:   pulumi.String("A"),
//! 			Ttl:    pulumi.Int(3600),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Add a record requiring a data map
//! 		_, err = cloudflare.NewRecord(ctx, "_sipTls", &cloudflare.RecordArgs{
//! 			ZoneId: pulumi.Any(_var.Cloudflare_zone_id),
//! 			Name:   pulumi.String("_sip._tls"),
//! 			Type:   pulumi.String("SRV"),
//! 			Data: &cloudflare.RecordDataArgs{
//! 				Service:  pulumi.String("_sip"),
//! 				Proto:    pulumi.String("_tls"),
//! 				Name:     pulumi.String("example-srv"),
//! 				Priority: pulumi.Int(0),
//! 				Weight:   pulumi.Int(0),
//! 				Port:     pulumi.Int(443),
//! 				Target:   pulumi.String("example.com"),
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		return nil
//! 	})
//! }
//! ```
//! ### Java
//! ```java
//! package generated_program;
//!
//! import com.pulumi.Context;
//! import com.pulumi.Pulumi;
//! import com.pulumi.core.Output;
//! import com.pulumi.cloudflare.Record;
//! import com.pulumi.cloudflare.RecordArgs;
//! import com.pulumi.cloudflare.inputs.RecordDataArgs;
//! import java.util.List;
//! import java.util.ArrayList;
//! import java.util.Map;
//! import java.io.File;
//! import java.nio.file.Files;
//! import java.nio.file.Paths;
//!
//! public class App {
//!     public static void main(String[] args) {
//!         Pulumi.run(App::stack);
//!     }
//!
//!     public static void stack(Context ctx) {
//!         // Add a record to the domain
//!         var example = new Record("example", RecordArgs.builder()        
//!             .zoneId(var_.cloudflare_zone_id())
//!             .name("example")
//!             .value("192.0.2.1")
//!             .type("A")
//!             .ttl(3600)
//!             .build());
//!
//!         // Add a record requiring a data map
//!         var _sipTls = new Record("_sipTls", RecordArgs.builder()        
//!             .zoneId(var_.cloudflare_zone_id())
//!             .name("_sip._tls")
//!             .type("SRV")
//!             .data(RecordDataArgs.builder()
//!                 .service("_sip")
//!                 .proto("_tls")
//!                 .name("example-srv")
//!                 .priority(0)
//!                 .weight(0)
//!                 .port(443)
//!                 .target("example.com")
//!                 .build())
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Add a record to the domain
//!   example:
//!     type: cloudflare:Record
//!     properties:
//!       zoneId: ${var.cloudflare_zone_id}
//!       name: example
//!       value: 192.0.2.1
//!       type: A
//!       ttl: 3600
//!   # Add a record requiring a data map
//!   _sipTls:
//!     type: cloudflare:Record
//!     properties:
//!       zoneId: ${var.cloudflare_zone_id}
//!       name: _sip._tls
//!       type: SRV
//!       data:
//!         service: _sip
//!         proto: _tls
//!         name: example-srv
//!         priority: 0
//!         weight: 0
//!         port: 443
//!         target: example.com
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/record:Record example <zone_id>/<record_id>
//! ```
//!

pub struct RecordArgs {
    /// Allow creation of this record in Terraform to overwrite an existing record, if any. This does not affect the ability to
    /// update the record in Terraform and does not prevent other resources within Terraform or manual changes outside Terraform
    /// from overwriting this record. **This configuration is not recommended for most environments**
    pub allow_overwrite: pulumi_wasm_rust::Output<Option<bool>>,
    /// Comments or notes about the DNS record. This field has no effect on DNS responses.
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// Map of attributes that constitute the record value. Conflicts with `value`.
    pub data: pulumi_wasm_rust::Output<Option<crate::types::RecordData>>,
    /// The name of the record.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The priority of the record.
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// Whether the record gets Cloudflare's origin protection.
    pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
    /// Custom tags for the DNS record.
    pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The TTL of the record.
    pub ttl: pulumi_wasm_rust::Output<Option<i32>>,
    /// The type of the record. Available values: `A`, `AAAA`, `CAA`, `CNAME`, `TXT`, `SRV`, `LOC`, `MX`, `NS`, `SPF`, `CERT`,
    /// `DNSKEY`, `DS`, `NAPTR`, `SMIMEA`, `SSHFP`, `TLSA`, `URI`, `PTR`, `HTTPS`, `SVCB`
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The value of the record.
    pub value: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct RecordResult {
    /// Allow creation of this record in Terraform to overwrite an existing record, if any. This does not affect the ability to
    /// update the record in Terraform and does not prevent other resources within Terraform or manual changes outside Terraform
    /// from overwriting this record. **This configuration is not recommended for most environments**
    pub allow_overwrite: pulumi_wasm_rust::Output<Option<bool>>,
    /// Comments or notes about the DNS record. This field has no effect on DNS responses.
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// The RFC3339 timestamp of when the record was created.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// Map of attributes that constitute the record value. Conflicts with `value`.
    pub data: pulumi_wasm_rust::Output<Option<crate::types::RecordData>>,
    /// The FQDN of the record.
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// A key-value map of string metadata Cloudflare associates with the record.
    pub metadata: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    /// The RFC3339 timestamp of when the record was last modified.
    pub modified_on: pulumi_wasm_rust::Output<String>,
    /// The name of the record.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The priority of the record.
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// Shows whether this record can be proxied.
    pub proxiable: pulumi_wasm_rust::Output<bool>,
    /// Whether the record gets Cloudflare's origin protection.
    pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
    /// Custom tags for the DNS record.
    pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The TTL of the record.
    pub ttl: pulumi_wasm_rust::Output<i32>,
    /// The type of the record. Available values: `A`, `AAAA`, `CAA`, `CNAME`, `TXT`, `SRV`, `LOC`, `MX`, `NS`, `SPF`, `CERT`,
    /// `DNSKEY`, `DS`, `NAPTR`, `SMIMEA`, `SSHFP`, `TLSA`, `URI`, `PTR`, `HTTPS`, `SVCB`
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The value of the record.
    pub value: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RecordArgs) -> RecordResult {
    let result = crate::bindings::pulumi::cloudflare::record::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::record::Args {
            allow_overwrite: args.allow_overwrite.get_inner(),
            comment: args.comment.get_inner(),
            data: args.data.get_inner(),
            name: args.name.get_inner(),
            priority: args.priority.get_inner(),
            proxied: args.proxied.get_inner(),
            tags: args.tags.get_inner(),
            ttl: args.ttl.get_inner(),
            type_: args.type_.get_inner(),
            value: args.value.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    RecordResult {
        allow_overwrite: crate::into_domain(result.allow_overwrite),
        comment: crate::into_domain(result.comment),
        created_on: crate::into_domain(result.created_on),
        data: crate::into_domain(result.data),
        hostname: crate::into_domain(result.hostname),
        metadata: crate::into_domain(result.metadata),
        modified_on: crate::into_domain(result.modified_on),
        name: crate::into_domain(result.name),
        priority: crate::into_domain(result.priority),
        proxiable: crate::into_domain(result.proxiable),
        proxied: crate::into_domain(result.proxied),
        tags: crate::into_domain(result.tags),
        ttl: crate::into_domain(result.ttl),
        type_: crate::into_domain(result.type_),
        value: crate::into_domain(result.value),
        zone_id: crate::into_domain(result.zone_id),
    }
}
