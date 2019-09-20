use clap::{App, Arg};
use mail::{client::send, Mail, MailError, Result};

const DEFAULT_SEND: &str = "Alice";
const DEFAULT_FROM: &str = "alice@crepe.fr";

fn main() -> Result<()> {
    env_logger::init();

    let mail = from_arguments().ok_or(MailError::MalformedInput)?;
    send(&mail)
}

fn from_arguments() -> Option<Mail> {
    let matches = App::new("Mail client")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("send")
                .long("send")
                .short("s")
                .value_name("NAME")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("from")
                .long("from")
                .short("f")
                .value_name("MAILBOX")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("recv")
                .long("recv")
                .short("r")
                .value_name("NAME")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("to")
                .long("to")
                .short("t")
                .value_name("MAILBOX")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("image")
                .long("img")
                .short("i")
                .value_name("FILE")
                .takes_value(true)
                .help("supports only jpeg files")
        )
        .arg(Arg::with_name("body").value_name("BODY").required(true))
        .get_matches();

    let send = matches.value_of("send").unwrap_or(DEFAULT_SEND).to_string();
    let from = matches.value_of("from").unwrap_or(DEFAULT_FROM).to_string();
    let recv = matches.value_of("recv")?.to_string();
    let to = matches.value_of("to")?.to_string();
    let body = matches.value_of("body")?.to_string();
    let image = matches.value_of("image").map(|s| s.into());

    Some(Mail {
        send,
        from,
        recv,
        to,
        body,
        image,
    })
}
