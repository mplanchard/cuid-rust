# Legacy compat for folks not on nix with flakes.
#
# flake-compat reads the flake and provides shellNix (for nix-shell) and defaultNix
# (for nix-build). We only need shell here, since we're only using nix for the
# dev environment atm.
(import (
  fetchTarball {
    url = "https://github.com/edolstra/flake-compat/archive/99f1c2157fba4bfe6211a321fd0ee43199025dbf.tar.gz";
    sha256 = "0x2jn3vrawwv9xp15674wjz9pixwjyj3j771izayl962zziivbx2"; }
) {
  src =  ./.;
}).shellNix
