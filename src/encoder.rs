// use tokio::fs::File;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
// use tokio::time::{self, Duration};
use image::DynamicImage;
// use rand::Rng;
// use rand::distributions::Alphanumeric;
use steganography::encoder::Encoder;
// use steganography::util;
use tokio::fs;


fn image_to_bytes(img: &DynamicImage) -> Vec<u8> {
    let mut bytes = Vec::new();
    img.write_to(&mut bytes, image::ImageOutputFormat::PNG).unwrap();
    bytes
}

// fn bytes_to_image(bytes: &[u8]) -> Result<DynamicImage, image::ImageError> {
//     image::load_from_memory(bytes)
// }
// //Temp Functions

// async fn load_image_async(path: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {
//     let mut file = File::open(path).await?;
//     let mut buffer = Vec::new();
//     let bytes_read  = tokio::io::AsyncReadExt::read_to_end(&mut file, &mut buffer).await?;
//     let img = image::load_from_memory(&buffer)?;

//     Ok(img)
// }

// async fn save_image_async(path: &str, img: &DynamicImage) -> Result<(), Box<dyn std::error::Error>> {
//     let mut output_file = File::create(path).await?;
//     let img_bytes = image_to_bytes(img);
//     output_file.write_all(&img_bytes).await?;
//     Ok(())
// }


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let default_img_path = "src/pic.jpg".to_string();

    let default_img = steganography::util::file_as_dynamic_image(default_img_path);
    // let default_img = load_image_async(default_img_path).await?;

    // let user_img_path = "src/img2.jpg".to_string();
    // let user_img = steganography::util::file_as_dynamic_image(user_img_path);
    // let user_img = load_image_async(user_img_path).await?;

    // let user_img_bytes = image_to_bytes(&user_img);
   
   
    //The code to be integrated starts from here

    let user_img_bytes = fs::read("src/img2.jpg").await?;
    
    let (tx, mut rx) = mpsc::channel(1000);
    
    let (tx2, mut rx2) = mpsc::channel(1000);

    tx.send(user_img_bytes).await.expect("Failed to send image bytes");

    let encryption_handle = tokio::spawn(async move {

        let user_img_bytes = rx.recv().await.unwrap();
        

        // Create a steganography encoder
        let encoder = Encoder::new(&user_img_bytes, default_img);   
        let encoded_img = encoder.encode_alpha();

        // let encoder_output: DynamicImage = DynamicImage::ImageRgba8(encoded_img);
        // let encoded_img_bytes = encoded_img.into_raw();
        // Send the encoded image bytes back to the main thread

        // let encoded_img_bytes = image_to_bytes(&encoder_output);
        tx2.send(encoded_img).await.expect("Failed to send stegnogrified image");
    });

    // Receive the encrypted image bytes from the main thread
    let encoded_img = rx2.recv().await.expect("Failed to receive stegnogrified image");

    
    let output_path = "src/encrypted_test.jpeg".to_string();
    steganography::util::save_image_buffer(encoded_img, output_path);

    // let output_img = bytes_to_image(&encoded_img_bytes).expect("Failed to convert bytes to image");
    // save_image_async(output_path, &output_img).await?;

    encryption_handle.await.expect("Thread join failed");

    
    println!("Image stegnogrified and saved successfully!");

    Ok(())

}