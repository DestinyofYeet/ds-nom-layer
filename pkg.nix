{ rustPlatform, lib, ... }:

rustPlatform.buildRustPackage {
  pname = "ds-nom-layer";
  version = "1.0";

  src = ./.;

  cargoHash = "sha256-nq+XRVpVa6SmbA1rh++POAH+Ouc22sHIgqS85rTXVzU=";

  meta = with lib; {
    description = "Determinate-systems nom output layer";
    mainProgram = "ds-nom-layer";
    license = licenses.gpl2;
    platforms = platforms.all;
  };
}
