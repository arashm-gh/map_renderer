{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  packages = with pkgs; [
    rustup
    glfw
    cmake
    clang
    wayland
    xorg.xorgproto
    xorg.libX11.dev
    xorg.libXrandr.dev
    xorg.libXinerama.dev  # Add this line for the Xinerama headers
  ];
  
  LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
    libGL
    xorg.libXrandr
    xorg.libXinerama  # This line might already be present for the runtime library
    xorg.libXcursor
    xorg.libXi
    xorg.libX11
  ];
  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
}
