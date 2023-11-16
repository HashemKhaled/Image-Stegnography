use image::{GenericImageView, Rgba};
use steganography::encoder::Encoder;
use tokio::fs;
use tokio::sync::mpsc;

struct Image {
    dims: (u32,u32),
    data: Vec<u8>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let default_img_path = "def_img.jpg".to_string();
    let default_img = steganography::util::file_as_dynamic_image(default_img_path);

    println!("Default image dimensions: {:?}", default_img.dimensions());

    let secret_bytes = fs::read("img2.jpg").await?;
    
    let (tx, mut rx) = mpsc::channel(1000);
    let (tx2, mut rx2) = mpsc::channel(1000);

    tx.send(secret_bytes).await.expect("Failed to send image bytes");

    let encryption_handle = tokio::spawn(async move {

        let secret_bytes = rx.recv().await.unwrap();
        
        // Create a steganography encoder
        let encoder = Encoder::new(&secret_bytes, default_img);   
        let encoded_img = encoder.encode_alpha();

        let image = Image{ dims : encoded_img.dimensions(),
            data:encoded_img.into_raw()
        };

        println!("Encoded image dimensions: {:?}", image.dims);
        tx2.send(image).await.expect("Failed to send stegnogrified image");
    });

    // Receive the encrypted image bytes from the main thread
    let endoed_image = rx2.recv().await.expect("Failed to receive stegnogrified image");

    
    let output_path = "encoded_img.jpg".to_string();
    
    let image_buffer: image::ImageBuffer<Rgba<u8>, Vec<u8>> = image::ImageBuffer::from_raw(endoed_image.dims.0, endoed_image.dims.1, endoed_image.data).unwrap();
    steganography::util::save_image_buffer(image_buffer.clone(), output_path);


    encryption_handle.await.expect("Thread join failed");
    println!("Image stegnogrified and saved successfully!");

    Ok(())

}