/*
By: <Your Name Here>
Date: 2026-06-08
Program Details: <Program Description Here>
*/

use crate::modules::label::Label;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;

//this is the guessinhg screen

pub async fn run(
    uvholding: i32,
    emfholding: i32,
    tempholding: i32,
    boxholding: i32,
    emf: i32,
    sbox: i32,
    temp: i32,
    uv: i32,
    tm: TextureManager,
) -> (String, i32, i32, i32, i32, i32, i32, i32, i32, TextureManager) {
    let mut img_final = StillImage::new("", 2550.0, 1468.0, 0.0, 0.0, true, 1.0).await;
    img_final.set_preload(tm.get_preload("assets/fr.png").unwrap());


    let uvholding1 = uvholding;
let emfholding1 = emfholding;
let tempholding1=tempholding;
let boxholding1=boxholding;
    let sbox1 = sbox;
    let temp1 = temp;
    let uv1 = uv;
    let emf1 = emf;
    let mut book = 0;

    let mut img_book = StillImage::new("", 2550.0, 1468.0, 0.0, 0.0, true, 1.0).await;
    img_book.set_preload(tm.get_preload("assets/infobookforingame.png").unwrap());
    img_book.clear();

    let mut btn_g1 = TextButton::new(805.0, 1075.0, 100.0, 100.0, "Twin", BLUE, GREEN, 30);
    let mut btn_g2 = TextButton::new(1075.0, 1075.0, 100.0, 100.0, "Gallu", BLUE, GREEN, 30);
    let mut btn_g3 = TextButton::new(1275.0, 1075.0,100.0, 100.0, "TheMICINCICNC", BLUE, GREEN, 30);
    let mut btn_g4 = TextButton::new(1475.0, 1075.0, 100.0, 100.0, "Spirt", BLUE, GREEN, 30);
    let mut lbl_youwin = Label::new("", 500.0, 50.0, 30);
     lbl_youwin.with_colors(WHITE, Some(DARKGRAY));

    loop {
        clear_background(BLACK);        
        img_final.draw();
        if btn_g1.click() && temp1 == 1 && sbox1 == 1 {
            lbl_youwin.set_text("You win");
            btn_g1.enabled = false;
            btn_g2.enabled = false;
            btn_g3.enabled = false;
            btn_g4.enabled = false;
        } else if btn_g1.click() {
            lbl_youwin.set_text("you lose");
            btn_g1.enabled = false;
            btn_g2.enabled = false;
            btn_g3.enabled = false;
            btn_g4.enabled = false;
        }

        if btn_g2.click() && emf1 == 1 && uv1 == 1 {
            lbl_youwin.set_text("You win");
            btn_g1.enabled = false;
            btn_g2.enabled = false;
            btn_g3.enabled = false;
            btn_g4.enabled = false;
        } else if btn_g2.click() {
            lbl_youwin.set_text("you lose");
            btn_g1.enabled = false;
            btn_g2.enabled = false;
            btn_g3.enabled = false;
            btn_g4.enabled = false;
        }

        if btn_g3.click() && emf1 == 1 && temp1 == 1 {
            lbl_youwin.set_text("You win");
            btn_g1.enabled = false;
            btn_g2.enabled = false;
            btn_g3.enabled = false;
            btn_g4.enabled = false;
        } else if btn_g3.click() {
            lbl_youwin.set_text("you lose");
            btn_g1.enabled = false;
            btn_g2.enabled = false;
            btn_g3.enabled = false;
            btn_g4.enabled = false;
        }

        if btn_g4.click() && sbox1 == 1 && uv1 == 1 {
            lbl_youwin.set_text("You win");
            btn_g1.enabled = false;
            btn_g2.enabled = false;
            btn_g3.enabled = false;
            btn_g4.enabled = false;
        } else if btn_g4.click() {
            lbl_youwin.set_text("you lose");
            btn_g1.enabled = false;
            btn_g2.enabled = false;
            btn_g3.enabled = false;
            btn_g4.enabled = false;
        }

        if is_key_pressed(KeyCode::J) {
            if book == 0 {
                book = 1;
                img_book.set_preload(tm.get_preload("assets/infobookforingame.png").unwrap());
            } else if book == 1 {
                book = 0;
                img_book.clear();
            }
        }

 lbl_youwin.draw();
        img_book.draw();
        next_frame().await;
    }
}
