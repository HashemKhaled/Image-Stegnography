use tokio::sync::mpsc;
use image::DynamicImage;
use image::GenericImageView;
use steganography::decoder::Decoder;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs::File as std_File;

fn image_to_bytes(img: &DynamicImage) -> Vec<u8> {
    let mut bytes = Vec::new();
    img.write_to(&mut bytes, image::ImageOutputFormat::PNG).unwrap();
    bytes
}

fn bytes_to_image(bytes: &[u8]) -> Result<DynamicImage, image::ImageError> {
    image::load_from_memory(bytes)
}

// async fn save_image_async(path: &str, img: &DynamicImage) -> Result<(), Box<dyn std::error::Error>> {
//     let mut output_file = File::create(path).await?;
//     let img_bytes = image_to_bytes(img);
//     output_file.write_all(&img_bytes).await?;
//     Ok(())
// }


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let img_path = "src/encrypted_test.jpeg".to_string();
    let img_buffer = steganography::util::file_as_image_buffer(img_path);
    
    let decoder = Decoder::new(img_buffer);
    let decoded_bytes = decoder.decode_alpha();

    let output_path = "src/decoded_test.png";
    
    let _ = fs::write(output_path, decoded_bytes).await?;


    // let mut output_file = std_File::create(output_path).unwrap();
    // steganography::util::bytes_to_file(&decoded_bytes, &output_file);



    // let decoded_img = bytes_to_image(&decoded_bytes).unwrap();
    // let output_path = "src/decoded_test.png";

    // save_image_async(output_path, &decoded_img).await?;

    println!("Image decoded and saved successfully!");

    Ok(())
}