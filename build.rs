// Without app manifest (https://en.wikipedia.org/wiki/Manifest_file) the app would not be able to write
//  in some file locations
// Source: https://github.com/nabijaczleweli/rust-embed-resource

fn main() {
    #[cfg(target_os = "windows")]
    embed_resource::compile("integra-cli-manifest.rc");
}