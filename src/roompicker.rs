/*
By: <Your Name Here>
Date: 2026-06-08
Program Details: <Program Description Here>
*/

use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::preload_image::TextureManager;


pub async fn run(uvholding: i32,emfholding: i32,tempholding:i32,boxholding:i32,emf:i32,sbox:i32,temp:i32,uv:i32, tm: TextureManager ) -> (String,i32,i32,i32,i32,i32,i32,i32,i32, TextureManager) {
let img_minimap = StillImage::new("assets/minimap.png",2550.0,1468.0,0.0,0.0,true,1.0,).await;
let btn_fronthall = TextButton::new( 785.0,575.0,500.0,320.0,"fronthall",BLUE,GREEN,30);
let btn_cararea = TextButton::new( 1350.0,520.0,365.0,480.0,"carzone",BLUE,GREEN,30);
let btn_bathroom = TextButton::new( 1350.0,375.0,375.0,125.0,"bathroom",BLUE,GREEN,30);
let btn_food = TextButton::new( 1150.0,115.0,375.0,325.0,"food",BLUE,GREEN,30);
let btn_bedroom = TextButton::new( 750.0,350.0,400.0,200.0,"bedroom",BLUE,GREEN,30);


    loop {
        clear_background(BLACK);
       

if btn_fronthall.click() {return ("fronthall".to_string(),uvholding,emfholding,tempholding,boxholding,emf,sbox,temp,uv, tm);}
if btn_cararea.click() {return ("cararea".to_string(),uvholding,emfholding,tempholding,boxholding,emf,sbox,temp,uv, tm);}
if btn_food.click() {return ("food".to_string(),uvholding,emfholding,tempholding,boxholding,emf,sbox,temp,uv, tm);}
if btn_bedroom.click() {return ("bedroom".to_string(),uvholding,emfholding,tempholding,boxholding,emf,sbox,temp,uv, tm);} 
if btn_bathroom.click() { return ("bathroom".to_string(),uvholding,emfholding,tempholding,boxholding,emf,sbox,temp,uv, tm);} 
img_minimap.draw();  
next_frame().await;
}
}