;; Copied from https://ifcoltransglinks.wordpress.com/2024/01/24/creating-a-webassembly-component-with-wat-and-wit/
(component
  (core module $LengthCoreWasm
    (func (export "length") (param $ptr i32) (param $len i32) (result i32)
      local.get $len
    )
    (memory (export "mem") 1)
    (func (export "realloc") (param i32 i32 i32 i32) (result i32)
      i32.const 0
    )
    (@custom "pulumi_gestalt_provider::random" (after data) "{\"version\": \"4.15.0\"}")
    (@custom "pulumi_gestalt_provider::docker" (after data) "{\"version\": \"4.5.3\", \"pluginDownloadURL\": \"https://example.com\"}")
  )
  (core instance $length_instance (instantiate $LengthCoreWasm))
  (func (export "length") (param "input" string) (result u32)
    (canon lift
      (core func $length_instance "length")
      (memory $length_instance "mem")
      (realloc (func $length_instance "realloc"))
    )
  )
)