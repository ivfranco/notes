extern crate openssl;

use openssl::x509::{X509, X509Name};
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;
use openssl::rsa::Rsa;
use openssl::nid::Nid;
use openssl::error::ErrorStack;

fn problem_20_4() -> Result<(), ErrorStack> {
    let rsa = Rsa::generate(4096)?;
    let pk = PKey::from_rsa(rsa)?;

    let mut name_builder = X509Name::builder()?;
    name_builder.append_entry_by_nid(Nid::COMMONNAME, "foo")?;
    let name = name_builder.build();

    let mut builder = X509::builder()?;
    builder.set_version(2)?;
    builder.set_subject_name(&name)?;
    builder.set_issuer_name(&name)?;
    builder.set_pubkey(&pk)?;
    builder.sign(&pk, MessageDigest::sha256())?;
    let cert = builder.build();

    let pem = cert.to_pem()?;
    println!("{}", String::from_utf8(pem).unwrap());

    Ok(())
}

fn main() {
    problem_20_4();
}
