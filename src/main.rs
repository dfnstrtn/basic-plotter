pub mod common;
pub mod file_read;
pub mod coordinates;
pub mod rendering_system;
pub mod server;
use rusttype ::{point,Font,Scale};

use std::thread;
use std::sync::mpsc;


use std::io::Write;
use std::path::Path;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::collections::HashMap;
use std::collections::HashSet;
use image::{RgbImage,Rgb};
const WIDTH:u32 = 1000;
const HEIGHT:u32 = 1000;
const TEXT_COLOR:(u8,u8,u8) = (70,70,70);
//const x_zero:f32=WIDTH as f32/2.;
//const y_zero:f32=HEIGHT as f32/2.;
use common::*;


fn main(){
    
    let mut plotting_keys:HashMap<String,((u8,u8,u8),Vec<(f32,f32)>)> = HashMap::new();
    
    let (tx,rx) = mpsc::channel::<DataFormat>();
    let tx = tx.clone();
    thread::spawn(move||{
        server::handle_connections(tx,"127.0.0.1:3333");
    });



    let sdl_context=sdl2::init().unwrap();
    let video_subsystem=sdl_context.video().unwrap();
    let window=video_subsystem.window("antcolonyrobot",WIDTH,HEIGHT)
    .position_centered().build().unwrap();
    
    let mut canvas=window.into_canvas().build().unwrap();
    let mut events=sdl_context.event_pump().unwrap(); 
    let mut running = true;
    let mut new_coords = coordinates::Coords2D::new(WIDTH as f32,HEIGHT as f32,1000.,WIDTH as f32);
    new_coords.transform.set_abs_translation(WIDTH as f32/2.,HEIGHT as f32/2.);
    let mut transamount = 200.;

    let mut points: Vec<(f32,f32)> = Vec::new();

    while(running){
        
        let p_data = rx.try_recv();
        if let Ok(z) = p_data{
            let name =  z.name;
            match plotting_keys.get_mut(&name){
                Some(v) => {
                    v.1.push((z.points[0],z.points[1]));
                }
                None=>{
                    let color = (z.color[0],z.color[1],z.color[2]);
                    plotting_keys.insert(name,( color, vec![(z.points[0],z.points[1])] ));
                }
            };
        }


        for event in events.poll_iter(){
            match event{
                //Event::Quit{..}|Event::KeyDown{keycode:Some{KeyCode::Escape}}
                Event::Quit{..}|Event::KeyDown{keycode : Some(Keycode::Escape),..}=>{running=false},
                Event::KeyDown{keycode : Some(Keycode::Z ),..}=>{
                    //let s = new_coords.transform.get_scale();
                   new_coords.scale(2.);
                    
                },

                Event::KeyDown{keycode : Some(Keycode::X),..}=>{
                    //let s = new_coords.transform.get_scale();
                    new_coords.scale(0.5);
                },


                Event::KeyDown{keycode : Some(Keycode::A),..}=>{
                   
                    let t = new_coords.transform.get_abs_translation();
                    new_coords.transform.set_abs_translation(t.0+transamount,t.1);
                },


                Event::KeyDown{keycode : Some(Keycode::D),..}=>{
                    
                    let t = new_coords.transform.get_abs_translation();
                    new_coords.transform.set_abs_translation(t.0-transamount,t.1);

                },


                Event::KeyDown{keycode : Some(Keycode::W),..}=>{
                   
                    let t = new_coords.transform.get_abs_translation();
                    new_coords.transform.set_abs_translation(t.0,transamount+t.1);
                    },


                Event::KeyDown{keycode : Some(Keycode::S),..}=>{
                    
                    let t = new_coords.transform.get_abs_translation();
                    new_coords.transform.set_abs_translation(t.0,t.1-transamount);
                },


                Event::KeyDown{keycode : Some(Keycode::R),..}=>{
                    
                    let t = new_coords.transform.get_rotation();
                    new_coords.transform.set_rotation(t + 0.1);
                },
            
                

                Event::KeyDown{keycode : Some(Keycode::C),..}=>{
                        new_coords.transform.set_abs_translation(WIDTH as f32/2.,HEIGHT as f32/2.);
                },

                _=>(), 
            }
        }
        canvas.set_draw_color(sdl2::pixels::Color::BLACK);
        canvas.clear();
        
        for m in plotting_keys.iter().enumerate(){
            let key = m.1.0;
            let color = m.1.1.0;
            let start = (WIDTH as i32 - 200, HEIGHT as i32 -20 - 25*(m.0 as i32) );
            render_plot_key(&mut canvas,&key,start,16.,color);
            for n in m.1.1.1.iter(){
                new_coords.render_filled_rect_color((n.0,n.1) , color, &mut canvas);
            }
        }
         
        new_coords.render_coord_system(&mut canvas);
        canvas.present();
    }
}


// points are absolute points on the screen 
fn render_plot_key(canvas:&mut sdl2::render::Canvas<sdl2::video::Window>,data:&String, mut start:(i32,i32), f_size:f32,color:(u8,u8,u8)){
       rendering_system::render_rect_filled(canvas,sdl2::rect::Rect::new(start.0,start.1,f_size as u32, f_size as u32),color);
       
       start.0+=3*f_size as i32;

       rendering_system::render_text(start, canvas,data.clone(),f_size,(250,250,250)) 
}
