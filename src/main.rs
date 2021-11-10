use aco::ACOgrid;
use aco::test_aco;
use rusttype ::{point,Font,Scale};
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





fn main(){
    //let grid = ACOgrid::new(20,20,20.0);
    //grid.display();
    test_aco();
}







fn mani() {
    let grid =ACOgrid::new(20,20,20.0);
    grid.display();
    println!("Hello, world!");


   	let sdl_context=sdl2::init().unwrap();
    let video_subsystem=sdl_context.video().unwrap();
    let window=video_subsystem.window("antcolonyrobot",1024,768)
    .position_centered().build().unwrap();
    
    let mut canvas=window.into_canvas().build().unwrap();
    let mut events=sdl_context.event_pump().unwrap(); 
    let mut running = true;
    let mut data = String::from("YOU SUCK"); 
	let mut x=0;
	let mut	y=100;



    while(running){
        for event in events.poll_iter(){
            match event{
                //Event::Quit{..}|Event::KeyDown{keycode:Some{KeyCode::Escape}}
                Event::Quit{..}|Event::KeyDown{keycode : Some(Keycode::Escape),..}=>{running=false},
				Event::Quit{..}|Event::KeyDown{keycode : Some(Keycode::Backspace),..}=>{match data.pop(){Some(a)=>(),None => data=String::new()}},
        			_=>(), 
            }
        }
        let pixel_data=get_raster(&data);
		let width = pixel_data.1;
		x=0;
		y=0;
		for j in 0..pixel_data.0.len(){
			render(&mut canvas,(x,y,pixel_data.0[ (y as usize)*width+ (x as usize)]));	
			x+=1;
		    if x==width as u32{
			    x=0;
			    y+=1;
			    //println!("");
		    }
		//handle.write_all(&pixel_data[j*width..(j+1)*width]).unwrap();
		//print!("{}",pixel_data[ j]);
		//handle.write_all(b"\n").unwrap();
		}	
        canvas.present();
    }
}






pub fn render(canvas:&mut sdl2::render::Canvas<sdl2::video::Window>,data:(u32,u32,u8)){
  		let m =data.2;
		canvas.set_draw_color(sdl2::pixels::Color::RGB(m,m,m));
  		canvas.draw_point(sdl2::rect::Point::new(data.0 as i32,data.1 as i32));
  }






pub fn get_raster(word:&String)->(Vec<u8>,usize){
	  
	let font_path= Path::new("slkscr.ttf");
    let data = std::fs::read(&font_path).unwrap();
    let font = Font::try_from_vec(data).unwrap_or_else(||{panic!("can't make font")});
    let height = 16f32;
    let pixel_height = height.ceil() as usize;
    let scale = Scale{x:height,y:height};
    let v_metrics= font.v_metrics(scale);
    let offset = point (0.0 , v_metrics.ascent);
	let glyphs: Vec<_> = font.layout(word.as_str(),scale,offset).collect();
	let width = glyphs.iter().rev().map(|g| g.position().x as f32
                                        + g.unpositioned().h_metrics().advance_width)
        .next().unwrap_or(0.0).ceil() as usize;

    //println!("w : {} h: {}",width,pixel_height);

    let mut pixel_data = vec![0u8;width*pixel_height];
    let mapping_scale = 128 as f32;
    for g in glyphs{
        if let Some(bb) = g.pixel_bounding_box(){
            g.draw(
            |x,y,v|{
                let i = (v*mapping_scale) as u8;
                //let c =mapping.get(i).cloned().unwrap_or(b'$');
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;

                if x>=0 &&x<width as i32 && y>=0 && y<pixel_height as i32{
                    let x = x as usize;
                    let y = y as usize;
                    pixel_data[(x + y*width)]=i;

                }})
           }
        }
	  (pixel_data,width)	  
  }







