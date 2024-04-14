//> using dep com.lihaoyi::os-lib:0.9.3

case class Provider(name: String, version: String)

val providers = Seq(
  Provider("docker", "4.5.3"),
  Provider("random", "4.15.0")
)

@main
def main(): Unit = {

  for (provider <- providers) {
    println(provider);
    val schema = os
      .proc(
        "pulumi",
        "package",
        "get-schema",
        s"${provider.name}@${provider.version}"
      )
      .spawn()
    os.write.over(
      os.pwd / "providers" / s"${provider.name}.json",
      schema.stdout
    )
  }

  updateCargoToml()
  updateJustfile()
}

def updateCargoToml(): Unit = {
  val content = os.read(os.pwd / "Cargo.toml")
  val replacement = providers
    .flatMap(provider => {
      Seq(
        s"providers/pulumi_wasm_provider_${provider.name}",
        s"providers/pulumi_wasm_provider_${provider.name}_rust"
      )
    })
    .map(line => s"    \"${line}\",\n")
    .mkString("")
  val startMarker = "# DO NOT EDIT - START"
  val endMarker = "# DO NOT EDIT - END"
  val newContent =
    replaceBetweenMarkers(content, startMarker, endMarker, replacement)

  os.write.over(os.pwd / "Cargo.toml", newContent)
}

def updateJustfile(): Unit = {
  val content = os.read(os.pwd / "justfile")
  val replacement = providers
    .flatMap(provider => {
      Seq(
        s"cargo run -p cargo-pulumi-gen -- gen-provider --remove true --schema providers/${provider.name}.json --output providers/pulumi_wasm_provider_${provider.name}",
        s"cargo run -p cargo-pulumi-gen -- gen-rust     --remove true --schema providers/${provider.name}.json --output providers/pulumi_wasm_provider_${provider.name}_rust"
      )
    })
    .map(line => s"    ${line}\n")
    .mkString("")
  val startMarker = "# DO NOT EDIT - START\nregenerate-providers:"
  val endMarker = "# DO NOT EDIT - END"
  val newContent =
    replaceBetweenMarkers(content, startMarker, endMarker, replacement)

  os.write.over(os.pwd / "justfile", newContent)
}

def replaceBetweenMarkers(
    source: String,
    startMarker: String,
    endMarker: String,
    replacement: String
): String = {
  source.replaceAll(
    s"(?s)(.*${startMarker}\n).*(${endMarker}.*)",
    s"$$1${replacement}$$2"
  )
}
