{writeShellApplication}:
writeShellApplication {
  name = "weave";

  text = builtins.readFile ../../weave;
}
