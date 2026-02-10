#![allow(unused)]

use axum::Router;
use axum::response::Html;
use axum::extract::State;
use axum::routing::get;
use std::net::SocketAddr;
use axum::response::IntoResponse;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use dotenv::dotenv;
mod models;
use models::{create_record, get_record, get_latest_records};


// use opencv::{
//     prelude::*,
//     videoio,
//     highgui,
//     objdetect,
//     imgcodecs,
//     core,
//     types
// }; // Note, the namespace of OpenCV is changed (to better or worse). It is no longer one enormous.
// use opencv::imgcodecs::IMWRITE_JPEG_QUALITY;

// cargo run and go to 
// http://127.0.0.1:8085/hello
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");


    let routes = Router::new()
        .route("/", get(home_page))
        .route("/records", get(get_records_html))
        .route("/barcode_scanner", get(get_records_html))
        .with_state(pool);
    


  
    let addr = SocketAddr::from(([127, 0, 0, 1], 8085));
    println!("->> LISTENING on http:/{addr}\n");

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
    // endregion:  Start Server

    Ok(())
    
}

async fn home_page() -> Html<String> {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Inventory Tracker</title>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                    height: 100vh;
                    margin: 0;
                    background-color: #f5f5f5;
                }
                h1 {
                    color: #333;
                    margin-bottom: 30px;
                }
                .button {
                    background-color: #4CAF50;
                    color: white;
                    padding: 15px 32px;
                    text-align: center;
                    text-decoration: none;
                    display: inline-block;
                    font-size: 16px;
                    border-radius: 4px;
                    cursor: pointer;
                    transition: background-color 0.3s;
                }
                .button:hover {
                    background-color: #45a049;
                }
            </style>
        </head>
        <body>
            <h1>Inventory Tracker</h1>
            <a href="/records" class="button">View Inventory</a>
            <p></p>
            <a href="/barcode_scanner" class="button">Scan barcode</a>
        </body>
        </html>
    "#.to_string();
    
    Html(html)
}


async fn barcode_scanner() -> Html<String> {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Inventory Tracker</title>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                    height: 100vh;
                    margin: 0;
                    background-color: #f5f5f5;
                }
                h1 {
                    color: #333;
                    margin-bottom: 30px;
                }
                .button {
                    background-color: #4CAF50;
                    color: white;
                    padding: 15px 32px;
                    text-align: center;
                    text-decoration: none;
                    display: inline-block;
                    font-size: 16px;
                    border-radius: 4px;
                    cursor: pointer;
                    transition: background-color 0.3s;
                }
                .button:hover {
                    background-color: #45a049;
                }
            </style>
        </head>
        <body>
            <h1>Barcode Scanner</h1>
            <a href="/" class="button">Capture barcode</a>
  
        </body>
        </html>
    "#.to_string();
    
    Html(html)
}

async fn get_records_html(State(pool): State<PgPool>) -> Html<String> {
    match get_latest_records(&pool, 100).await {
        Ok(records) => {
        let mut html = String::from(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Latest Records</title>
                <style>
                    body { font-family: Arial, sans-serif; margin: 20px; }
                    button {
                    background-color: #4CAF50;
                    color: white;
                    padding: 15px 32px;
                    text-align: center;
                    text-decoration: none;
                    display: inline-block;
                    font-size: 16px;
                    border-radius: 4px;
                    cursor: pointer;
                    transition: background-color 0.3s;
                }
                    table { border-collapse: collapse; width: 100%; }
                    th, td { border: 1px solid #ddd; padding: 12px; text-align: left; }
                    th { background-color: #4CAF50; color: white; }
                    tr:nth-child(even) { background-color: #f2f2f2; }
                    tr:hover { background-color: #ddd; }
                </style>
            </head>
            <body>
                <a href="/" class="button">Home Page</a>
                <h1>Fixed Phones</h1>
                
                <table>
                    <tr>
                        <th>ID</th>
                        <th>Name</th>
                        <th>IMEI</th>
                        <th>Changed Parts</th>
                        <th>Created At</th>
                    </tr>
            "#
        );

        for record in records {
            html.push_str(&format!(
                r#"
                    <tr>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                    </tr>
                "#,
                record.id,
                record.name,
                record.imei,
                record.changed_parts.join(", "),
                record.created_ts.format("%Y-%m-%d %H:%M:%S")
            ));
        }

        html.push_str(
            r#"
                </table>
            </body>
            </html>
            "#
        );

        Html(html)
        }
        Err(e) => {
            Html(format!("<h1>Database error: {}</h1>", e))
        }
    }
}





// fn capture_imei() -> Result<()> { // Note, this is anyhow::Result
//     // Open a GUI window
//     let mut detector = objdetect::BarcodeDetector::default()?;
//     highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;
//     // Open the web-camera (assuming you have one)
//     let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
//     let mut frame = Mat::default(); // This array will store the web-cam data
//     // Read the camera
//     // and display in the window
//     // let mut res = core::Vector::<core::Point>::new();
//     // let mut qr_img = Mat::default();
//     let mut points = core::Vector::<core::Point>::new();
//     let mut straight_code = Mat::default();
//     let mut decoded_info = core::Vector::<String>::new();
//     loop {
//         cam.read(&mut frame)?;

//         // let  res = qr_detector.detect_and_decode(&frame, &mut decoded_info, &mut decoded_type, &mut points)?;


//         let found = detector.detect_and_decode(
//             &frame,
//             &mut points,
//             &mut straight_code
             
           
//         )?;
//         println!("data: {:?}", found);
       
        
        
        

//         let s = String::from_utf8_lossy(&found);
//         println!("{:?}", s);
    

//         highgui::imshow("window", &frame)?;

      


//         let key = highgui::wait_key(1)?;

//         if key == 113 { // quit with q
//             break;
//         }
//     }

// 	let  mut parms = opencv::core::Vector::default();
// 		parms.push(IMWRITE_JPEG_QUALITY);
// 		parms.push(95);
// 	opencv::imgcodecs::imwrite("./imei.png", &frame, &parms);

//     Ok(())
// }