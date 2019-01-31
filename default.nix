{pkgs ? import ./nix/pkgs.nix {}}:
{
    runtime = pkgs.callPackage ./runtime {};
}
