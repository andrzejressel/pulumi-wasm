#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheKeyFieldsUser {
    /// `true` - classifies a request as “mobile”, “desktop”, or “tablet” based on the User Agent; defaults to `false`.
    #[serde(rename = "deviceType")]
    pub r#device_type: Box<Option<bool>>,
    /// `true` - includes the client’s country, derived from the IP address; defaults to `false`.
    #[serde(rename = "geo")]
    pub r#geo: Box<Option<bool>>,
    /// `true` - includes the first language code contained in the `Accept-Language` header sent by the client; defaults to `false`.
    /// 
    /// Example:
    /// 
    /// <!--Start PulumiCodeChooser -->
    /// ### Typescript
    /// ```typescript
    /// import * as pulumi from "@pulumi/pulumi";
    /// import * as cloudflare from "@pulumi/cloudflare";
    /// 
    /// // Unrealistic example with all features used
    /// const foobar = new cloudflare.PageRule("foobar", {
    ///     zoneId: _var.cloudflare_zone_id,
    ///     target: `${_var.cloudflare_zone}/app/*`,
    ///     priority: 1,
    ///     actions: {
    ///         cacheKeyFields: {
    ///             cookie: {
    ///                 checkPresences: ["wordpress_test_cookie"],
    ///             },
    ///             header: {
    ///                 checkPresences: ["header_present"],
    ///                 excludes: ["origin"],
    ///                 includes: [
    ///                     "api-key",
    ///                     "dnt",
    ///                 ],
    ///             },
    ///             host: {
    ///                 resolved: true,
    ///             },
    ///             queryString: {
    ///                 ignore: true,
    ///             },
    ///             user: {
    ///                 deviceType: false,
    ///                 geo: true,
    ///                 lang: true,
    ///             },
    ///         },
    ///     },
    /// });
    /// ```
    /// ### Python
    /// ```python
    /// import pulumi
    /// import pulumi_cloudflare as cloudflare
    /// 
    /// # Unrealistic example with all features used
    /// foobar = cloudflare.PageRule("foobar",
    ///     zone_id=var["cloudflare_zone_id"],
    ///     target=f"{var['cloudflare_zone']}/app/*",
    ///     priority=1,
    ///     actions=cloudflare.PageRuleActionsArgs(
    ///         cache_key_fields=cloudflare.PageRuleActionsCacheKeyFieldsArgs(
    ///             cookie=cloudflare.PageRuleActionsCacheKeyFieldsCookieArgs(
    ///                 check_presences=["wordpress_test_cookie"],
    ///             ),
    ///             header=cloudflare.PageRuleActionsCacheKeyFieldsHeaderArgs(
    ///                 check_presences=["header_present"],
    ///                 excludes=["origin"],
    ///                 includes=[
    ///                     "api-key",
    ///                     "dnt",
    ///                 ],
    ///             ),
    ///             host=cloudflare.PageRuleActionsCacheKeyFieldsHostArgs(
    ///                 resolved=True,
    ///             ),
    ///             query_string=cloudflare.PageRuleActionsCacheKeyFieldsQueryStringArgs(
    ///                 ignore=True,
    ///             ),
    ///             user=cloudflare.PageRuleActionsCacheKeyFieldsUserArgs(
    ///                 device_type=False,
    ///                 geo=True,
    ///                 lang=True,
    ///             ),
    ///         ),
    ///     ))
    /// ```
    /// ### C#
    /// ```csharp
    /// using System.Collections.Generic;
    /// using System.Linq;
    /// using Pulumi;
    /// using Cloudflare = Pulumi.Cloudflare;
    /// 
    /// return await Deployment.RunAsync(() => 
    /// {
    ///     // Unrealistic example with all features used
    ///     var foobar = new Cloudflare.PageRule("foobar", new()
    ///     {
    ///         ZoneId = @var.Cloudflare_zone_id,
    ///         Target = $"{@var.Cloudflare_zone}/app/*",
    ///         Priority = 1,
    ///         Actions = new Cloudflare.Inputs.PageRuleActionsArgs
    ///         {
    ///             CacheKeyFields = new Cloudflare.Inputs.PageRuleActionsCacheKeyFieldsArgs
    ///             {
    ///                 Cookie = new Cloudflare.Inputs.PageRuleActionsCacheKeyFieldsCookieArgs
    ///                 {
    ///                     CheckPresences = new[]
    ///                     {
    ///                         "wordpress_test_cookie",
    ///                     },
    ///                 },
    ///                 Header = new Cloudflare.Inputs.PageRuleActionsCacheKeyFieldsHeaderArgs
    ///                 {
    ///                     CheckPresences = new[]
    ///                     {
    ///                         "header_present",
    ///                     },
    ///                     Excludes = new[]
    ///                     {
    ///                         "origin",
    ///                     },
    ///                     Includes = new[]
    ///                     {
    ///                         "api-key",
    ///                         "dnt",
    ///                     },
    ///                 },
    ///                 Host = new Cloudflare.Inputs.PageRuleActionsCacheKeyFieldsHostArgs
    ///                 {
    ///                     Resolved = true,
    ///                 },
    ///                 QueryString = new Cloudflare.Inputs.PageRuleActionsCacheKeyFieldsQueryStringArgs
    ///                 {
    ///                     Ignore = true,
    ///                 },
    ///                 User = new Cloudflare.Inputs.PageRuleActionsCacheKeyFieldsUserArgs
    ///                 {
    ///                     DeviceType = false,
    ///                     Geo = true,
    ///                     Lang = true,
    ///                 },
    ///             },
    ///         },
    ///     });
    /// 
    /// });
    /// ```
    /// ### Go
    /// ```go
    /// package main
    /// 
    /// import (
    /// 	"fmt"
    /// 
    /// 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
    /// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
    /// )
    /// 
    /// func main() {
    /// 	pulumi.Run(func(ctx *pulumi.Context) error {
    /// 		// Unrealistic example with all features used
    /// 		_, err := cloudflare.NewPageRule(ctx, "foobar", &cloudflare.PageRuleArgs{
    /// 			ZoneId:   pulumi.Any(_var.Cloudflare_zone_id),
    /// 			Target:   pulumi.String(fmt.Sprintf("%v/app/*", _var.Cloudflare_zone)),
    /// 			Priority: pulumi.Int(1),
    /// 			Actions: &cloudflare.PageRuleActionsArgs{
    /// 				CacheKeyFields: &cloudflare.PageRuleActionsCacheKeyFieldsArgs{
    /// 					Cookie: &cloudflare.PageRuleActionsCacheKeyFieldsCookieArgs{
    /// 						CheckPresences: pulumi.StringArray{
    /// 							pulumi.String("wordpress_test_cookie"),
    /// 						},
    /// 					},
    /// 					Header: &cloudflare.PageRuleActionsCacheKeyFieldsHeaderArgs{
    /// 						CheckPresences: pulumi.StringArray{
    /// 							pulumi.String("header_present"),
    /// 						},
    /// 						Excludes: pulumi.StringArray{
    /// 							pulumi.String("origin"),
    /// 						},
    /// 						Includes: pulumi.StringArray{
    /// 							pulumi.String("api-key"),
    /// 							pulumi.String("dnt"),
    /// 						},
    /// 					},
    /// 					Host: &cloudflare.PageRuleActionsCacheKeyFieldsHostArgs{
    /// 						Resolved: pulumi.Bool(true),
    /// 					},
    /// 					QueryString: &cloudflare.PageRuleActionsCacheKeyFieldsQueryStringArgs{
    /// 						Ignore: pulumi.Bool(true),
    /// 					},
    /// 					User: &cloudflare.PageRuleActionsCacheKeyFieldsUserArgs{
    /// 						DeviceType: pulumi.Bool(false),
    /// 						Geo:        pulumi.Bool(true),
    /// 						Lang:       pulumi.Bool(true),
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
    /// ### Java
    /// ```java
    /// package generated_program;
    /// 
    /// import com.pulumi.Context;
    /// import com.pulumi.Pulumi;
    /// import com.pulumi.core.Output;
    /// import com.pulumi.cloudflare.PageRule;
    /// import com.pulumi.cloudflare.PageRuleArgs;
    /// import com.pulumi.cloudflare.inputs.PageRuleActionsArgs;
    /// import com.pulumi.cloudflare.inputs.PageRuleActionsCacheKeyFieldsArgs;
    /// import com.pulumi.cloudflare.inputs.PageRuleActionsCacheKeyFieldsCookieArgs;
    /// import com.pulumi.cloudflare.inputs.PageRuleActionsCacheKeyFieldsHeaderArgs;
    /// import com.pulumi.cloudflare.inputs.PageRuleActionsCacheKeyFieldsHostArgs;
    /// import com.pulumi.cloudflare.inputs.PageRuleActionsCacheKeyFieldsQueryStringArgs;
    /// import com.pulumi.cloudflare.inputs.PageRuleActionsCacheKeyFieldsUserArgs;
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
    ///         // Unrealistic example with all features used
    ///         var foobar = new PageRule("foobar", PageRuleArgs.builder()        
    ///             .zoneId(var_.cloudflare_zone_id())
    ///             .target(String.format("%s/app/*", var_.cloudflare_zone()))
    ///             .priority(1)
    ///             .actions(PageRuleActionsArgs.builder()
    ///                 .cacheKeyFields(PageRuleActionsCacheKeyFieldsArgs.builder()
    ///                     .cookie(PageRuleActionsCacheKeyFieldsCookieArgs.builder()
    ///                         .checkPresences("wordpress_test_cookie")
    ///                         .build())
    ///                     .header(PageRuleActionsCacheKeyFieldsHeaderArgs.builder()
    ///                         .checkPresences("header_present")
    ///                         .excludes("origin")
    ///                         .includes(                        
    ///                             "api-key",
    ///                             "dnt")
    ///                         .build())
    ///                     .host(PageRuleActionsCacheKeyFieldsHostArgs.builder()
    ///                         .resolved(true)
    ///                         .build())
    ///                     .queryString(PageRuleActionsCacheKeyFieldsQueryStringArgs.builder()
    ///                         .ignore(true)
    ///                         .build())
    ///                     .user(PageRuleActionsCacheKeyFieldsUserArgs.builder()
    ///                         .deviceType(false)
    ///                         .geo(true)
    ///                         .lang(true)
    ///                         .build())
    ///                     .build())
    ///                 .build())
    ///             .build());
    /// 
    ///     }
    /// }
    /// ```
    /// ### YAML
    /// ```yaml
    /// resources:
    ///   # Unrealistic example with all features used
    ///   foobar:
    ///     type: cloudflare:PageRule
    ///     properties:
    ///       zoneId: ${var.cloudflare_zone_id}
    ///       target: ${var.cloudflare_zone}/app/*
    ///       priority: 1
    ///       actions:
    ///         cacheKeyFields:
    ///           cookie:
    ///             checkPresences:
    ///               - wordpress_test_cookie
    ///           header:
    ///             checkPresences:
    ///               - header_present
    ///             excludes:
    ///               - origin
    ///             includes:
    ///               - api-key
    ///               - dnt
    ///           host:
    ///             resolved: true
    ///           queryString:
    ///             ignore: true
    ///           user:
    ///             deviceType: false
    ///             geo: true
    ///             lang: true
    /// ```
    /// <!--End PulumiCodeChooser -->
    #[serde(rename = "lang")]
    pub r#lang: Box<Option<bool>>,
}
