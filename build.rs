use embed_manifest::embed_manifest_file;

fn main() {
    embed_manifest_file("killmewindowsibegchu.exe.manifest").expect("unable to embed manifest file");
}