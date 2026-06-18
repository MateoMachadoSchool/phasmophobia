/*
By: <Mateo machado>
Date: 2026-06-08
Program Details: <Program Description Here>
*/

mod modules;

mod mainmenu;
mod vanscreen;
mod cararea;
mod roompicker;
mod fronthall;
mod bathroom;
mod bedroom;
mod food;
mod fr;
use miniquad::date;
use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
use crate::modules::preload_image::LoadingScreenOptions;

fn window_conf() -> Conf {
    Conf {
        window_title: "Program Name".to_owned(),
        window_width: 2550,
        window_height: 1500,
        fullscreen: true,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut tm = TextureManager::new();

    let loading_options = LoadingScreenOptions {
       title: Some("Mateo Machado Game".to_string()),
       background_color: RED,
       bar_fill_color: GOLD,
       // Use default values for other options
       ..Default::default()
   };

    tm.preload_with_loading_screen(&["assets/bathroom.png","assets/fr.png", "assets/bathroomdoorbox.png", "assets/bathroomhitboxs.png", "assets/bedroom.png", "assets/bedroomdoorhitbox.png", "assets/bedroomhitbox.png", "assets/cararea.png", "assets/carhitboxes.png", "assets/cardoors.png", "assets/door.png", "assets/doorimagebox.png", "assets/doorwithuv.png", "assets/emfreader.png", "assets/Fooddoors.png", "assets/foodhitboxs.png", "assets/foodroom.png", "assets/fronthall_image.png", "assets/frontroomdoorshitbox.png", "assets/frontroomwalhitboxs.png", "assets/hotbar.png", "assets/infobookforingame.png", "assets/mainmenu.png", "assets/mainmenuinfopage1to2.png", "assets/mainmenuinfopage3to4.png", "assets/minimap.png", "assets/player.png", "assets/spirtbox.png", "assets/temps.png", "assets/uvstick.png", "assets/vaninside.png", ], Some(loading_options)).await;

    rand::srand(date::now() as u64);
    let mut current_screen = "mainmenu".to_string();
    let mut last_switch = get_time() - 0.02;
    let mut uvholding = 0; 
    let mut boxholding = 0;
    let mut emfholding =0;
    let mut tempholding =0;
  let ghostt = rand::gen_range(1, 5);
  println!("{}",ghostt);
   let mut emf =0;
      let mut uv =0;
   let mut sbox =0;
      let mut temp =0;
      if ghostt==1{emf=0; sbox=1; temp=1; uv=0;}
      if ghostt==2{emf=1; sbox=0; temp=0; uv=1;}
      if ghostt==3{emf=1; sbox=0; temp=1; uv=0;}
      if ghostt==4{emf=0; sbox=1; temp=0; uv=1;}

      



    loop {
        if get_time() - last_switch > 0.01 {
            (current_screen,uvholding,emfholding,tempholding,boxholding,emf,sbox,temp,uv, tm) = match current_screen.as_str() {
                "mainmenu" => mainmenu::run(uvholding,emfholding,tempholding,boxholding,emf,sbox,temp,uv, tm).await,
                "vanscreen" => vanscreen::run(uvholding,emfholding,tempholding,boxholding,emf,sbox,temp,uv, tm).await,
                "roompicker" => roompicker::run(uvholding,emfholding,tempholding,boxholding,emf,sbox,temp,uv, tm).await,
                "fronthall" => fronthall::run(uvholding,emfholding,tempholding,boxholding,emf,sbox,temp,uv, tm).await,
                "bathroom" => bathroom::run(uvholding,emfholding,tempholding,boxholding,emf,sbox,temp,uv, tm).await,
                "food" => food::run(uvholding,emfholding,tempholding,boxholding,emf,sbox,temp,uv, tm).await,
                "bedroom" => bedroom::run(uvholding,emfholding,tempholding,boxholding,emf,sbox,temp,uv, tm).await,
                "cararea" => cararea::run(uvholding,emfholding,tempholding,boxholding,emf,sbox,temp,uv,tm).await,
                "fr" => fr::run(uvholding,emfholding,tempholding,boxholding,emf,sbox,temp,uv,tm).await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}