/*
By: <Mateo machado>
Date: 2026-06-08
Program Details: <Program Description Here>
*/
use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::collision::check_collision;
use crate::modules::text_input::TextInput;
use crate::modules::label::Label;
use crate::modules::preload_image::TextureManager;

pub async fn run(uvholding: i32,emfholding: i32,tempholding:i32,boxholding:i32,emf:i32,sbox:i32,temp:i32,uv:i32, tm: TextureManager) -> (String,i32,i32,i32,i32,i32,i32,i32,i32, TextureManager) {
let uvholding1 = uvholding;
let emfholding1 = emfholding;
let tempholding1=tempholding;
let boxholding1=boxholding;
let sbox1 =sbox;
let temp1=temp;
let uv1 =uv;
let emf1 =emf;
let mut img_fronthall = StillImage::new("",2000.0,1068.0,0.0,100.0,true,1.0,).await;
img_fronthall.set_preload(tm.get_preload("assets/fronthall_image.png").unwrap());

let mut img_book = StillImage::new("",2550.0,1468.0,0.0,0.0,true,1.0,).await;
img_book.set_preload(tm.get_preload("assets/infobookforingame.png").unwrap());

let mut img_hitboxfront = StillImage::new("",2000.0,1068.0,0.0,100.0,true,1.0,).await;
img_hitboxfront.set_preload(tm.get_preload("assets/frontroomwalhitboxs.png").unwrap());

let mut img_hitboxdoors = StillImage::new("",2000.0,1068.0,0.0,100.0,true,1.0,).await;
img_hitboxdoors.set_preload(tm.get_preload("assets/frontroomdoorshitbox.png").unwrap());

let mut img_doorimagebox = StillImage::new("",750.0,750.0,1675.0,300.0,true,1.0,).await;
img_doorimagebox.set_preload(tm.get_preload("assets/doorimagebox.png").unwrap());

let mut img_hotbar = StillImage::new("",1200.0,1200.0,1450.0,600.0,true,1.0,).await;
img_hotbar.set_preload(tm.get_preload("assets/hotbar.png").unwrap());

let mut img_player = StillImage::new("",70.0,70.0,1000.0,950.0,true,1.0,).await;
img_player.set_preload(tm.get_preload("assets/player.png").unwrap());

let mut img_uv = StillImage::new("",300.0,275.0,2175.0,1075.0,true,1.0,).await;
img_uv.set_preload(tm.get_preload("assets/uvstick.png").unwrap());

let mut img_emf = StillImage::new("",300.0,300.0,1975.0,1050.0,true,1.0,).await;
img_emf.set_preload(tm.get_preload("assets/emfreader.png").unwrap());

let mut img_door = StillImage::new("",550.0,500.0,1775.0,425.0,true,1.0,).await;
img_door.set_preload(tm.get_preload("assets/door.png").unwrap());

let mut img_sbox = StillImage::new("",300.0,300.0,1825.0,1075.0,true,1.0,).await;
img_sbox.set_preload(tm.get_preload("assets/spirtbox.png").unwrap());

let mut img_temps = StillImage::new("",300.0,300.0,1625.0,1100.0,true,1.0,).await;
img_temps.set_preload(tm.get_preload("assets/temps.png").unwrap());

let mut btn_boxinput = TextButton::new( 1875.0,1075.0,300.0,300.0,"box",BLUE,GREEN,30);
let mut btn_temprun = TextButton::new( 1705.0,1250.0,100.0,100.0,"temp",BLUE,GREEN,30);
let mut txt_boxinput = TextInput::new(2050.0, 1200.0, 75.0, 40.0, 25.0);
let mut lbl_tempoutput = Label::new("", 1750.0, 1175.0, 30);
let mut lbl_emfoutput = Label::new("", 2165.0, 1200.0, 30);
let mut raot = rand::gen_range(2500, 1000);
let mut tempwiththeer: i32=0;
let mut emftimer=raot;
let mut emfr=0;
let mut rc =0;
let mut book=0;
txt_boxinput.set_prompt("Input");
    txt_boxinput.set_prompt_color(DARKGRAY);
    txt_boxinput.set_background_color(ORANGE);
      lbl_emfoutput.with_colors(WHITE, Some(DARKGRAY));
txt_boxinput.set_position(150000.0, 150.0);
   txt_boxinput.set_max_chars(0);
btn_boxinput.enabled = false;
    img_book.clear();
    img_uv.clear();
    img_door.clear();
    img_sbox.clear();
    img_emf.clear();
    img_temps.clear();
   
    const MOVE_SPEED: f32 = 250.0;
    loop { 
        clear_background(BLACK);
        img_hitboxfront.draw();
        img_hitboxdoors.draw();
        img_fronthall.draw();
        


//item setup
if uvholding1 ==1{
img_uv.set_image("assets/uvstick.png").await;
}

if boxholding1== 1{
btn_boxinput.enabled = true;
   txt_boxinput.set_max_chars(1000);
img_sbox.set_preload(tm.get_preload("assets/spirtbox.png").unwrap());
txt_boxinput.set_position(1915.0, 1200.0);}

if emfholding1== 1{
img_emf.set_preload(tm.get_preload("assets/emfreader.png").unwrap());}

if tempholding1== 1{
btn_temprun.enabled = true;
img_temps.set_preload(tm.get_preload("assets/temps.png").unwrap());}

//box proof
if btn_boxinput.click() {
txt_boxinput.set_text("");
rc = rand::gen_range(1, 7);
if rc==1&& sbox1==1{
        txt_boxinput.set_prompt("Behind");
    txt_boxinput.set_prompt_color(DARKGRAY);
   
}
}

//temps proof
if btn_temprun.click()&&temp1==0&&tempholding1==1{
tempwiththeer = rand::gen_range(0, 11); 
lbl_tempoutput.set_text( tempwiththeer.to_string());
}
if btn_temprun.click()&&temp1==1&&tempholding1==1{
tempwiththeer = rand::gen_range(-5, 11);
lbl_tempoutput.set_text( tempwiththeer.to_string());
}


// Direction to move in
    let mut move_dir = vec2(0.0, 0.0);

    // Keyboard input  
   

    if is_key_down(KeyCode::Right) {
        move_dir.x += 2.0;}
  
    if is_key_down(KeyCode::Left) {
        move_dir.x -= 2.0;
    }
    if is_key_down(KeyCode::Down) {
        move_dir.y += 2.0;
    }
    if is_key_down(KeyCode::Up) {
        move_dir.y -= 2.0;
    }

     if is_key_down(KeyCode::D) {
        move_dir.x += 2.0;
    }
    if is_key_down(KeyCode::A) {
        move_dir.x -= 2.0;
    }
    if is_key_down(KeyCode::S) {
        move_dir.y += 2.0;
    }
    if is_key_down(KeyCode::W) {
        move_dir.y -= 2.0;
    }

    if !is_key_down(KeyCode::E){
img_door.clear();
    }

    // Normalize the movement to prevent faster diagonal movement
    if move_dir.length() > 0.0 {
        move_dir = move_dir.normalize();
    }

    // Apply movement based on frame time
    let movement = move_dir * MOVE_SPEED * get_frame_time();

    // Save old position in case of collision
    let old_pos = img_player.pos();

// Move X first
    if movement.x != 0.0 {
        img_player.set_x(img_player.get_x() + movement.x);
        if check_collision(&img_player, &img_hitboxfront, 1) {
            img_player.set_x(old_pos.x); // Undo if collision happens
           }}

    // Move Y next
    if movement.y != 0.0 {
        img_player.set_y(img_player.get_y() + movement.y);
        if check_collision(&img_player, &img_hitboxfront, 1)  {
            img_player.set_y(old_pos.y); // Undo if collision happens 
            }}
//timer
raot= raot -1;
emftimer = raot;

//emf proof
if emftimer<=0&&emf1==1&&emfholding1==1{
    emfr = rand::gen_range(2, 6);
    raot = rand::gen_range(250, 1001);
       lbl_emfoutput.set_text( emfr.to_string());
        println!("with proof{}",emfr);}

else if emftimer<=0&&emf1==0&&emfholding1==1{
    emfr = rand::gen_range(1, 5);
    raot = rand::gen_range(250, 1001);
    lbl_emfoutput.set_text( emfr.to_string());
     println!("Emf with out proof{}",emfr);
}

else if emfholding1<=0{
      raot = rand::gen_range(250, 1001);
}

//uv proof
let doorgrabbed = check_collision(&img_player, &img_hitboxdoors, 1);
if doorgrabbed && uvholding1==1 && uv1==1 && is_key_down(KeyCode::E){
img_door.set_preload(tm.get_preload("assets/doorwithuv.png").unwrap());

} else if doorgrabbed ==true && is_key_down(KeyCode::E)&& uvholding1==0 || doorgrabbed ==true && is_key_down(KeyCode::E)&& uvholding1==1 && uv1==0 {
img_door.set_preload(tm.get_preload("assets/door.png").unwrap());
}
 
 if is_key_down(KeyCode::F) && doorgrabbed==true {
            return ("vanscreen".to_string(),uvholding,emfholding,tempholding,boxholding,emf,sbox,temp,uv, tm);}

        img_doorimagebox.draw();
        img_hotbar.draw();
        lbl_emfoutput.draw();
        img_door.draw();
        img_uv.draw();
        img_temps.draw();
        lbl_tempoutput.draw();
        img_emf.draw();
        img_sbox.draw();
        txt_boxinput.draw();
        img_player.draw();
        img_book.draw();
         if is_key_pressed(KeyCode::J) {if book==0{book=1;img_book.set_preload(tm.get_preload("assets/infobookforingame.png").unwrap());}else if book==1{book=0;img_book.clear();}}
        next_frame().await
    }
}