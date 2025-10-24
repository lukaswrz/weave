{
  lib,
  rustPlatform,
}:

rustPlatform.buildRustPackage {
  pname = "weave";
  version = "0.1.0";

  src = lib.cleanSource ./.;

  cargoHash = "sha256-dkg6nMG2z2b9chjEsI08uB+AsPu2sr7U2DNo7lV+nyM=";

  meta = {
    description = "A simple link manager";
    license = lib.licenses.gpl3Only;
    mainProgram = "weave";
  };
}
