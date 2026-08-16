{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    #    flake-root.url = "github:srid/flake-root";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        # Read the file relative to the flake's root
        overrides = (builtins.fromTOML (builtins.readFile (self + "/rust-toolchain.toml")));
        qtEnv = with pkgs.qt6; env "qt-custom-${qtbase.version}" [
          qtbase
          qmake
          qttools
          qtwayland
          qtdeclarative
          qtlanguageserver
          qt3d
          qtquick3d
          qtwayland
          wrapQtAppsHook
        ];
      in
      {
        devShells.default = pkgs.mkShell rec {
          #inputsFrom = [ pkgs.config.flake-root.devShell ];
          nativeBuildInputs = with pkgs; [
            pkg-config
            libglvnd
            egl-wayland
            egl-x11
            eglexternalplatform
            wayland
            qtEnv
            cmake
            makeWrapper
          ];
          buildInputs = with pkgs; [
            qt6.qtbase
            qt6.wrapQtAppsHook
            clang
            llvmPackages.bintools
            rustup
          ];

          RUSTC_VERSION = overrides.toolchain.channel;
          
          # https://github.com/rust-lang/rust-bindgen#environment-variables
          LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
          
          shellHook = ''
            export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
            export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
            export SHELL_IS_DEVSHELL="true"
            export QMAKE="$(qmake -query "QT_INSTALL_PREFIX")/bin/qmake"
            zshdir=$(mktemp -d)
            makeWrapper "$(type -p zsh)" "$zshdir/zsh" "''${qtWrapperArgs[@]}"
            exec "$zshdir/zsh"
          '';

          # Add precompiled library to rustc search path
          RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
            # add libraries here (e.g. pkgs.libvmi)
          ]);
          
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);

          
          # Add glibc, clang, glib, and other headers to bindgen search path
          BINDGEN_EXTRA_CLANG_ARGS =
          # Includes normal include path
          (builtins.map (a: ''-I"${a}/include"'') [
            # add dev libraries here (e.g. pkgs.libvmi.dev)
            pkgs.glibc.dev
          ])
          # Includes with special directory paths
          ++ [
            ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
            ''-I"${pkgs.glib.dev}/include/glib-2.0"''
            ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
          ];
        };
      }
    );
}
