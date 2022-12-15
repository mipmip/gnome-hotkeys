{ pkgs ? import <nixpkgs> { }
, lib ? import <nixpkgs/lib>
}:
let
in
pkgs.stdenv.mkDerivation rec {
  pname = "gtk-rust-template";
  version = "0.1.0";

  src = [ ./. ];

  cargoDeps = pkgs.rustPlatform.fetchCargoTarball {
    inherit src;
    name = "${pname}-${version}";
    hash = "sha256-VNMGeYikm98FtBOwaYpvxSvEcQdV7LnF6zQPUkIP/v0=";
  };

  nativeBuildInputs = with pkgs; [
    appstream-glib
    polkit
    gettext
    desktop-file-utils
    meson
    ninja
    pkg-config
    git
    wrapGAppsHook4
  ] ++ (with pkgs.rustPlatform; [
    cargoSetupHook
    rust.cargo
    rust.rustc
  ]);

  buildInputs = with pkgs; [
    gdk-pixbuf
    glib
    gtk4
    gtksourceview5
    libadwaita
    libxml2
    openssl
    wayland
    gnome.adwaita-icon-theme
    desktop-file-utils
  ];

#  postInstall = ''
#    wrapProgram $out/bin/nix-software-center --prefix PATH : '${lib.makeBinPath [ pkgs.gnome-console pkgs.sqlite ]}'
#  '';
}
