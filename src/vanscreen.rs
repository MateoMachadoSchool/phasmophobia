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
let mut uvholding1 = uvholding;
let mut emfholding1 = emfholding;
let mut tempholding1=tempholding;
let mut boxholding1=boxholding;
    let mut amountholding = 0; //amount of items you are holding

    let img_van = StillImage::new("assets/vaninside.png",2550.0,1468.0,0.0,0.0,true,1.0,).await;

let mut img_uvonvan = StillImage::new("assets/uvstick.png",750.0,750.0,0.0,300.0,true,1.0,).await;
let btn_uvgrab = TextButton::new( 300.0,350.0,200.0,650.0,"UVgrab",BLUE,GREEN,30);
let mut img_boxonvan = StillImage::new("assets/spirtbox.png",750.0,750.0,500.0,300.0,true,1.0,).await;
let btn_boxgrab = TextButton::new( 700.0,350.0,200.0,650.0,"boxgrab",BLUE,GREEN,30);

let mut img_temponvan = StillImage::new("assets/temps.png",750.0,750.0,1000.0,300.0,true,1.0,).await;
let btn_tempgrab = TextButton::new( 1250.0,350.0,200.0,650.0,"tempgrab",BLUE,GREEN,30);

let mut img_emfonvan = StillImage::new("assets/emfreader.png",750.0,750.0,1500.0,300.0,true,1.0,).await;
let btn_emfgrab = TextButton::new( 1800.0,350.0,200.0,650.0,"emfreader",BLUE,GREEN,30);
    loop {
        clear_background(DARKGRAY); 
        img_van.draw();
        

if uvholding1==1{
img_uvonvan.clear();
}

if emfholding1==1{
img_emfonvan.clear();
}

if boxholding1==1{
img_boxonvan.clear();
}

if tempholding1==1{
img_temponvan.clear();
}



//item grab code for uv
if btn_uvgrab.click() {
    if uvholding1 == 0 && amountholding == 2 {
    println!("nothing uv")
}
    if uvholding1 == 0 && amountholding <=1{
amountholding = amountholding +1;
uvholding1 = 1;
img_uvonvan.clear();}

else if uvholding1==1 {
uvholding1 = 0;
amountholding= amountholding -1;
img_uvonvan.set_image("assets/uvstick.png").await;}}


//item grab code for temp
if btn_tempgrab.click() {
    if tempholding1 == 0 && amountholding == 2 {
    println!("nothing temp")
}
    if tempholding1 == 0 && amountholding <=1{
amountholding = amountholding +1;
tempholding1 = 1;
img_temponvan.clear();}

else if tempholding1==1 {
tempholding1 = 0;
amountholding= amountholding -1;
img_temponvan.set_image("assets/temps.png").await;}}


//item grab code for box
if btn_boxgrab.click() {
    if boxholding1 == 0 && amountholding == 2 {
    println!("nothing box");
}
    if boxholding1 == 0 && amountholding <=1{
amountholding = amountholding +1;
boxholding1 = 1;
img_boxonvan.clear();}

else if boxholding1==1 {
boxholding1 = 0;
amountholding= amountholding -1;
img_boxonvan.set_image("assets/spirtbox.png").await;}}


//item grab code for emf
if btn_emfgrab.click() {
    if emfholding1 == 0 && amountholding == 2 {
    println!("nothing emf");
}
    if emfholding1 == 0 && amountholding <=1{
amountholding = amountholding +1;
emfholding1 = 1;
img_emfonvan.clear();}

else if emfholding1==1 {
emfholding1 = 0;
amountholding= amountholding -1;
img_emfonvan.set_image("assets/emfreader.png").await;}}

if is_key_pressed(KeyCode::Space) {
            return ("roompicker".to_string(),uvholding1,emfholding1,tempholding1,boxholding1,emf,sbox,temp,uv,tm);
        }

        if is_key_down(KeyCode::Enter) {return ("fr".to_string(),uvholding1,emfholding1,tempholding1,boxholding1,emf,sbox,temp,uv,tm);}

img_uvonvan.draw();
img_emfonvan.draw();
img_boxonvan.draw();
img_emfonvan.draw();
img_temponvan.draw();
        next_frame().await;
    }
}