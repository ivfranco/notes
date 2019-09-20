use crate::{resolver::resolve, Mail, MailError, Result};
use chrono::offset::Local;
use log::{error, info};
use std::{
    fs::File,
    io::{BufRead, BufReader, Write, Read},
    net::{Ipv4Addr, TcpStream},
};
use base64_stream::ToBase64Reader;

const SMTP_PORT: u16 = 25;
const BOUNDARY: &str = "frontier";
const BUF_LEN: usize = 0x80;

pub fn send(mail: &Mail) -> Result<()> {
    info!("Raw input data: {:?}", mail);
    let domain = extract_domain(&mail.to)?;
    info!("Resolving domain: {}", domain);
    let server = resolve(&domain)?;
    info!("Resolved to: {}", server);

    transmit(mail, server)?;

    Ok(())
}

fn extract_domain(mailbox: &str) -> Result<&str> {
    let at = mailbox.find('@').ok_or_else(|| {
        error!("Found no @ symbol in mailbox");
        MailError::MalformedInput
    })?;
    Ok(&mailbox[at + 1..])
}

fn transmit(mail: &Mail, server: Ipv4Addr) -> Result<()> {
    let local_domain = extract_domain(&mail.from)?;
    let mut stream = TcpStream::connect((server, SMTP_PORT))?;
    let mut reader = BufReader::new(stream.try_clone()?);

    // greeting message from server
    exit_on_negative_reply(&mut reader)?;

    info!("HELO {}", local_domain);
    write!(stream, "HELO {}\r\n", local_domain)?;
    exit_on_negative_reply(&mut reader)?;

    info!("MAIL FROM:<{}>", &mail.from);
    write!(stream, "MAIL FROM:<{}>\r\n", &mail.from)?;
    exit_on_negative_reply(&mut reader)?;

    info!("RCPT TO:<{}>", &mail.to);
    write!(stream, "RCPT TO:<{}>\r\n", &mail.to)?;
    exit_on_negative_reply(&mut reader)?;

    info!("DATA");
    write!(stream, "DATA\r\n")?;
    exit_on_negative_reply(&mut reader)?;

    transmit_multipart(&mut stream, mail)?;

    write!(stream, ".\r\n")?;
    exit_on_negative_reply(&mut reader)?;

    info!("QUIT");
    write!(stream, "QUIT\r\n")?;
    Ok(())
}

fn transmit_multipart(stream: &mut TcpStream, mail: &Mail) -> Result<()> {
    info!("Writing headers");
    write!(stream, "From: {}<{}>\r\n", &mail.send, &mail.from)?;
    write!(stream, "To: {}<{}>\r\n", &mail.recv, &mail.to)?;
    write!(stream, "Subject: Test\r\n")?;
    write!(
        stream,
        "Date: {}\r\n",
        Local::now().format("%d %b %Y %T %z")
    )?;
    write!(stream, "Mime-Version: 1.0\r\n")?;
    write!(stream, "Content-Type: multipart/mixed; boundary={}\r\n", BOUNDARY)?;
    write!(stream, "\r\n")?;

    write!(stream, "--{}\r\n", BOUNDARY)?;
    write!(stream, "Content-Type: text/plain\r\n")?;
    write!(stream, "\r\n")?;
    for line in mail.body.lines() {
        info!("BODY: {}", line);
        write!(stream, "{}\r\n", line)?;
    }

    if let Some(path) = &mail.image {
        info!("Sending image");
        let img = File::open(path)?;
        let name = path.file_name().expect("File name exist if it can be opened");
        write!(stream, "--{}\r\n", BOUNDARY)?;
        write!(stream, "Content-Type: image/jpeg; name={:?}\r\n", name)?;
        write!(stream, "Content-Transfer-Encoding: base64\r\n")?;
        write!(stream, "Content-Disposition: attachment; filename={:?}\r\n", name)?;
        write!(stream, "\r\n")?;

        let mut reader = ToBase64Reader::new(img);
        let mut buf = [0; BUF_LEN];
        let mut size = 0;
        loop {
            let len = reader.read(&mut buf)?;
            if len == 0 {
                break;
            }
            size += len;
            stream.write_all(&buf[..len])?;
            write!(stream, "\r\n")?;
        }
        info!("{} bytes sent", size);
    }

    write!(stream, "--{}--\r\n", BOUNDARY)?;

    // end of DATA section, a single dot on its own line
    Ok(())
}

struct SMTPReply {
    code: u16,
    message: String,
}

impl SMTPReply {
    fn from_reader<R>(mut reader: R) -> Result<Self>
    where
        R: BufRead,
    {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let space = line.find(' ').ok_or_else(|| {
            error!("Malformed response from SMTP server: no space");
            error!("{}", line);
            MailError::MalfomredResponse
        })?;
        let mut message = line.split_off(space);
        // last character is \r
        message.pop();
        let code = line.parse::<u16>().map_err(|_| {
            error!("Malformed response from SMTP server: missing reply code");
            error!("{}{}", line, message);
            MailError::MalfomredResponse
        })?;

        let reply = SMTPReply { code, message };
        info!("SMTP server reply: {:?}", reply);
        Ok(reply)
    }

    fn is_err(&self) -> bool {
        // 4yz  Transient Negative Completion reply
        // 5yz  Permanent Negative Completion reply
        self.code / 100 >= 4
    }
}

impl std::fmt::Debug for SMTPReply {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.code, self.message)
    }
}

fn exit_on_negative_reply<R>(reader: &mut R) -> Result<()>
where
    R: BufRead,
{
    let reply = SMTPReply::from_reader(reader)?;
    if reply.is_err() {
        error!("Negative server reply: {:?}", reply);
        Err(MailError::NegativeReply)
    } else {
        Ok(())
    }
}