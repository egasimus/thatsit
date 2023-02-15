{ pkgs ? import <nixpkgs> {}, ... }: let

  tools = with pkgs; [ pkg-config cmake xwayland mold mesa-demos wezterm ];

  libs  = with pkgs; [
    brotli
    bzip2
    dbus
    fontconfig
    freetype
    gnome.gdm
    libffi
    libglvnd
    libinput
    libpng
    libseat
    libxkbcommon
    mesa
    ncurses
    udev
    wayland
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    xorg.xcbutil
    xorg.xcbutilimage
    zlib
  ];

in pkgs.mkShell {

  name              = "thatsit";

  nativeBuildInputs = tools;

  buildInputs       = libs;

  LD_LIBRARY_PATH   = pkgs.lib.strings.makeLibraryPath libs;

}
