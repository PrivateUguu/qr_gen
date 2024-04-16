use qrcode::QrCode;
use image::Luma;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Args {
    /// Text to encode as a QR code.
    text: String
}

fn main() {
    let args =   Args::parse();
    let code = QrCode::new(args.text.as_bytes()).unwrap();
    let image = code.render::<Luma<u8>>().build();
    image.save("./qr_code.png").unwrap();
}
