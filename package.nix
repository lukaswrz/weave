{
  lib,
  rustPlatform,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "weave";
  version = "0.1.0";

  src = lib.cleanSource ./.;

  cargoLock.lockFile = "${finalAttrs.src}/Cargo.lock";

  meta = {
    description = "A simple link manager";
    license = lib.licenses.gpl3Only;
    mainProgram = "weave";
  };
})
