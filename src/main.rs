extern crate base64;
#[macro_use]
extern crate failure;
extern crate rust_sodium;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod internal;
use internal::{decrypt_file, encrypt_file, generate_key, transform_key};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "bincrypt", about = "Encrypt a file using XSalsa20-Poly1305!",
            author = "frk <hazefrk+dev@gmail.com>")]
struct Opt {
    /// A flag, true if used in the command line. Enables decryption mode.
    #[structopt(short = "d", long = "decrypt", help = "Activate decryption mode")]
    decrypt: bool,

    /// Optional parameter, the output file.
    #[structopt(short = "o", long = "output", help = "Output file")]
    output: Option<String>,

    /// Optional parameter, the base64 encoded encryption code.
    #[structopt(short = "k", long = "key", help = "Base64 encoded encryption key")]
    key: Option<String>,

    /// Needed parameter, the first on the command line.
    #[structopt(help = "Input file")]
    input: String,
}

fn main() {
    rust_sodium::init();

    let mut app_version = env!("GIT_PKG_VERSION_SEMVER");
    if app_version.is_empty() {
        app_version = env!("CARGO_PKG_VERSION");
    }

    let app = Opt::clap().version(app_version);
    let opt = Opt::from_clap(app.get_matches());

    let input = opt.input;
    let key_opt = opt.key.and_then(|k| transform_key(&k));

    if opt.decrypt {
        let output = opt.output.unwrap_or_else(|| input.clone() + ".dec");
        if let Some(key) = key_opt {
            if let Err(err) = decrypt_file(&input, &output, &key) {
                eprintln!("Error: {}", err);
            }
        } else {
            eprintln!("Error: Need valid encryption key to decrypt!")
        }
    } else {
        let key = key_opt.unwrap_or_else(generate_key);
        let output = opt.output.unwrap_or_else(|| input.clone() + ".enc");

        if let Err(err) = encrypt_file(&input, &output, &key) {
            eprintln!("Error: {}", err);
        }
    }
}
