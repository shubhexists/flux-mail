{ }:

let pkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/ef56e777fedaa4da8c66a150081523c5de1e0171.tar.gz") { overlays = [ (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz")) ]; };
in with pkgs;
  let
    APPEND_LIBRARY_PATH = "${lib.makeLibraryPath [  ] }";
    myLibraries = writeText "libraries" ''
      export LD_LIBRARY_PATH="${APPEND_LIBRARY_PATH}:$LD_LIBRARY_PATH"
      
    '';
  in
    buildEnv {
      name = "ef56e777fedaa4da8c66a150081523c5de1e0171-env";
      paths = [
        (runCommand "ef56e777fedaa4da8c66a150081523c5de1e0171-env" { } ''
          mkdir -p $out/etc/profile.d
          cp ${myLibraries} $out/etc/profile.d/ef56e777fedaa4da8c66a150081523c5de1e0171-env.sh
        '')
        curl gcc git openssl pkg-config
      ];
    }
