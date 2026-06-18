/*
By: <Your Name Here>
Date: 2026-06-08
Program Details: <Program Description Here>
*/

use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::preload_image::TextureManager;

//The controlls for the game are press space to start, and to leave the van to go to the room picker, on the room picker click on the room that you want to go into, pres j for guess book which lets you put you proof of the ghost in, e to look closer at doors, press f at doors to go back to the van

pub async fn run(uvholding: i32,emfholding: i32,tempholding:i32,boxholding:i32,emf:i32,sbox:i32,temp:i32,uv:i32, tm: TextureManager ) -> (String,i32,i32,i32,i32,i32,i32,i32,i32, TextureManager) {

    let mut img_mainmenu = StillImage::new("",2550.0,1468.0,0.0,0.0,true,1.0,).await;
    img_mainmenu.set_preload(tm.get_preload("assets/mainmenu.png").unwrap());
    loop {
        clear_background(RED);
        if is_key_pressed(KeyCode::Enter) {
           img_mainmenu.set_preload(tm.get_preload("assets/mainmenuinfopage1to2.png").unwrap());
            };

    if is_key_down(KeyCode::Space) {return ("vanscreen".to_string(),uvholding,emfholding,tempholding,boxholding,emf,sbox,temp,uv, tm);}    

    if is_key_down(KeyCode::Right) {
      img_mainmenu.set_preload(tm.get_preload("assets/mainmenuinfopage3to4.png").unwrap());}

img_mainmenu.draw();
        next_frame().await;
    }
}