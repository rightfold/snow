{stdenv, rustc}:
stdenv.mkDerivation {
    name = "snow-runtime-native";
    src = ./.;
    buildInputs = [rustc];
    phases = ["unpackPhase" "buildPhase" "installPhase" "fixupPhase"];
    buildPhase = ''
        CRATE_NAME='snowruntimenative'
        CRATE_ROOT='src/lib.rs'

        rustc                                                               \
            --crate-name "$CRATE_NAME"                                      \
            --crate-type staticlib                                          \
            -C opt-level=2 -C lto                                           \
            -C panic=abort                                                  \
            "$CRATE_ROOT"

        rustdoc                                                             \
            --crate-name "$CRATE_NAME"                                      \
            "$CRATE_ROOT"
    '';
    installPhase = ''
        mkdir -p "$out/lib" "$out/share"
        mv 'libsnowruntimenative.a' "$out/lib"
        mv 'doc' "$out/share"
    '';
}
