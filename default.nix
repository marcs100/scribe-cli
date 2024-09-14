{ pkgs ? import <nixpkgs> { }, fetchFromGitHub }:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "scribe-cli";
  version = "0.1.0";
  #cargoLock.lockFile = ./Cargo.lock;
  src = fetchFromGitHub {
    owner = "marcs100";
    repo = "scribe-cli";
    rev = "0.1.0";
    #hash = "1vyw1afgz82kl9p8ma4g51d56xx8bxnf86a4ndfvxij3axq26aix";
    #sha256 = "1vyw1afgz82kl9p8ma4g51d56xx8bxnf86a4ndfvxij3axq26aix";
    hash = "sha256-E0qFigubYEu3o+W3++XXabuUulOG7lmFjvdmOoqq4Mc=";
  };

  #cargoHash = "sha256-PSojcFdDxr5ds0QZ5GxfqHdTWiiPqIpuolOg/5wK3O8=";
  cargoSha256 = "sha256-JhLx3qvLRRJ0dHVGV5N4QBXok0kr2zztIHpAsH+2a/U=";

}
