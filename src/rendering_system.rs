use rusttype ::{point,Font,Scale};
use std::path::Path;




type PixelFormat = (u8,u8,u8); 

pub struct RenderingAbstractionLayer{
    pub total_pixels:usize,
    pub row_width:usize,
    pub pixels:Vec<PixelFormat>
}
impl RenderingAbstractionLayer{
    pub fn new(window_width:usize,window_height:usize)->RenderingAbstractionLayer{
        RenderingAbstractionLayer{
            total_pixels:window_width*window_height,
            row_width:window_width,
            pixels:(0..(window_width*window_height)).map(|m|{(0,0,0)}).collect::<Vec<PixelFormat>>()
        }
    }
    

    pub fn get_at(&self,x:usize,y:usize)->Option<PixelFormat>{
        let pixel = x+ y*self.row_width;
        if pixel>self.total_pixels{
            None
        }else{
            Some(  self.pixels[x+y*self.row_width] )
        }
    }

    pub fn set_at(&mut self,x:usize,y:usize,color:PixelFormat)->Result<usize,usize>{
        let pixel = x+y*self.row_width;
        match self.get_at(x,y){
            Some(v)=>{self.pixels[pixel] = color; Ok(pixel)}
            None=>{Err(pixel)}
        }
    }
    
    pub fn clear(&mut self){
        self.pixels.iter_mut().for_each(|m|{
            *m = (0,0,0)
        });
    }


    
    // manipulating it pixel by pixel is idiotic , but I have other important shite to do 
    pub fn render_to_screen(&self,canvas:&mut sdl2::render::Canvas<sdl2::video::Window>){
       let x_len = self.row_width;
       let y_len = self.total_pixels/self.row_width;
       for y in 0..y_len{
           for x in 0..x_len{
                let color = self.get_at(x,y).unwrap_or((0,0,0));
                canvas.set_draw_color(sdl2::pixels::Color::RGB(color.0,color.1,color.2));
                canvas.draw_point(sdl2::rect::Point::new(x as i32,y as i32));
           }
       }
    }
}




pub fn render_text(coord:(i32,i32), 
                   canvas:&mut sdl2::render::Canvas<sdl2::video::Window>,
                   text:String, 
                   font_size:f32,
                   font_color:(u8,u8,u8)){
        canvas.set_draw_color(sdl2::pixels::Color::RGB(font_color.0,font_color.1,font_color.2));
        let pixel_data=get_font_raster(&text,font_size);
		let width = pixel_data.1;
        let mut x=0;
		let mut y=0;
		for j in 0..pixel_data.0.len(){
            let col = pixel_data.0[ (y as usize)*width+ (x as usize)];
			render_point(canvas,(x+coord.0,y+coord.1,),(col,col,col) );	
			x+=1;
		    if x==width as i32{
			    x=0;
			    y+=1;
		    }
		}
}


pub fn render_line(start:(f32,f32),end:(f32,f32),color:(u8,u8,u8,),canvas:&mut sdl2::render::Canvas<sdl2::video::Window>){
    canvas.set_draw_color(sdl2::pixels::Color::RGB(color.0,color.1,color.2));
    canvas.draw_line(sdl2::rect::Point::new(start.0 as i32,start.1 as i32),sdl2::rect::Point::new(end.0 as i32 ,end.1 as i32));
}




pub fn render_point(canvas:&mut sdl2::render::Canvas<sdl2::video::Window>,point:(i32,i32), color:(u8,u8,u8)){
		canvas.set_draw_color(sdl2::pixels::Color::RGB(color.0,color.1,color.2));
  		canvas.draw_point(sdl2::rect::Point::new(point.0 as i32,point.1 as i32));
}


pub fn render_rect(canvas:&mut sdl2::render::Canvas<sdl2::video::Window>,rect:sdl2::rect::Rect, color:(u8,u8,u8)){
		canvas.set_draw_color(sdl2::pixels::Color::RGB(color.0,color.1,color.2));
  		canvas.draw_rect(rect);
  }



pub fn render_rect_filled(canvas:&mut sdl2::render::Canvas<sdl2::video::Window>,rect:sdl2::rect::Rect, color:(u8,u8,u8)){
		canvas.set_draw_color(sdl2::pixels::Color::RGB(color.0,color.1,color.2));
  		canvas.fill_rect(rect);
  }










// font stuff 
pub fn get_font_raster(word:&String, height:f32)->(Vec<u8>,usize){
	  
	let font_path= Path::new("Hack-Regular.ttf");
    let data = std::fs::read(&font_path).unwrap();

    let font = Font::try_from_vec(data).unwrap_or_else(||{panic!("can't make font")});
    let pixel_height = height.ceil() as usize;
    let scale = Scale{x:height,y:height};
    let v_metrics= font.v_metrics(scale);
    let offset = point (0.0 , v_metrics.ascent);
	let glyphs: Vec<_> = font.layout(word.as_str(),scale,offset).collect();
	let width = glyphs.iter().rev().map(|g| g.position().x as f32
                                        + g.unpositioned().h_metrics().advance_width)
        .next().unwrap_or(0.0).ceil() as usize;
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



