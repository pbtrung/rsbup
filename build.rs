extern crate gcc;

fn main() {
    gcc::Build::new()
               .file("src/pbkdf2/pbkdf2.c")
               .file("src/skein3fish/skein.c")
               .file("src/skein3fish/skeinApi.c")
               .file("src/skein3fish/skeinBlockNo3F.c")
               .file("src/skein3fish/threefish1024Block.c")
               .file("src/skein3fish/threefish256Block.c")
               .file("src/skein3fish/threefish512Block.c")
               .file("src/skein3fish/threefishApi.c")
               .include("src")
               .compile("rsbup");
}
