extern crate png;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let infile = &args[1];
    let outfile = &args[2];
    let (mut buf, width, height) = read_bytes(infile);
    transform_bytes(&mut buf);
    write_bytes(outfile, &buf, width, height)
}

fn read_bytes(infile_name: &String) -> (Vec<u8>, u32, u32) {
    // Make a decoder, return bytes, width and height.
    let decoder = png::Decoder::new(std::fs::File::open(infile_name).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    // Only accept 8-bit RGBA images.
    if info.color_type != png::ColorType::RGBA ||
       info.bit_depth != png::BitDepth::Eight {
        println!("Not my kind of PNG!");
        std::process::exit(1);
    }

    return (buf, info.width, info.height);
}

fn transform_bytes(buf: &mut Vec<u8>) -> () {
    // Iterator over all elements in the array, round down to even in place.
    for p in buf.iter_mut() {
        *p = (*p / 2) * 2;
    }
}

fn write_bytes(outfile_name: &String, buf: &Vec<u8>, width: u32, height: u32) -> () {
    // Now, write this out.
    use png::HasParameters; // As per PNG docs, needed for encoder.set

    let path = std::path::Path::new(outfile_name);
    let outfile = std::fs::File::create(path).unwrap();
    let ref mut wr = std::io::BufWriter::new(outfile);

    let mut encoder = png::Encoder::new(wr, width, height);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&buf).unwrap();
}
