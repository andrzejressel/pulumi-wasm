{ pkgs, lib, config, inputs, ... }:

{

  languages.go.enable = true;
  cachix.enable = false;

  packages = [
    pkgs.protobuf_25
  ];

}
