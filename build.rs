
extern crate gcc;

// Build script which is called before every 'cargo build'

fn main() {
    // Debug output only visible on panic!
    println!("Compiling libmovie_hash.a!");
    gcc::Config::new()
        .file("c_src/movie_hash.c")
        .compile("libmovie_hash.a");
}

