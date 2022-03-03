use crate::rendering_system;
use sdl2;
type Number  = f32;

pub struct Transform2D{
    scale:Number,
    rotation:Number,
    translation:(Number,Number),
    abs_translation:(Number,Number),
    mirror_x:bool,
    mirror_y:bool
}
impl Transform2D{
    pub fn new(translation_x:Number, 
               translation_y:Number,
               rotation:Number,
               scale:Number,
               mirror_x:bool,
               mirror_y:bool)->Transform2D{
        Transform2D{
            translation:(translation_x,translation_y),
            abs_translation:(0.,0.),
            rotation,
            scale,
            mirror_x,
            mirror_y
        }
    }

    pub fn set_translation(&mut self, translation_x:Number,translation_y:Number){
        self.translation = (translation_x,translation_y)
    }

    pub fn set_abs_translation(&mut self,translation_x:Number,translation_y:Number){
        self.abs_translation = (translation_x ,translation_y) 
    }



    pub fn get_abs_translation(&mut self)->(Number,Number){
        self.abs_translation 
    }

    pub fn set_rotation(&mut self, rotation:Number){
        self.rotation = rotation
    }

    pub fn set_scale(&mut self, scale:Number){
        self.scale = scale
    }
    



    pub fn get_scale(&mut self)->Number{
        self.scale
    }

    pub fn get_rotation(&mut self)->Number{
        self.rotation
    }

    pub fn get_translation(&mut self)->(Number,Number){
        self.translation
    }

    
    


    pub fn translate_x(&mut self, change:f32){
        let (x,y) = self.get_translation();
        self.set_translation(x+change,y);
    }

    pub fn translate_y(&mut self, change:f32){
        let (x,y) = self.get_translation();
        self.set_translation(x,y+change);
    }





    pub fn scale_factor(&mut self, scale:f32){
        let sc = self.get_scale();
        self.set_scale(scale*sc);
    }



    pub fn get_transform(&self,x:Number,y:Number)->(Number,Number){
        let new_x = (x - self.translation.0)*self.scale;
        let new_y = (y - self.translation.1)*self.scale;
        let ismirrored_x = match self.mirror_x{
            true=>-1.0,
            false=>1.0
        };
        


        let ismirrored_y = match self.mirror_y{
            true=>-1.0,
            false=>1.0
        };


        let rotated_x = new_x*self.rotation.cos() - new_y*self.rotation.sin();
        let rotated_y = new_x*self.rotation.sin() + new_y*self.rotation.cos();
        (rotated_x*ismirrored_x + self.abs_translation.0,rotated_y*ismirrored_y+self.abs_translation.1)
    }

}





/// Everything is mapped to a -1.0-1.0 in both x and y directions  
/// WIN_MAX specifies the maximum value in the x direction
/// It should be a tad more than windows size
pub struct Coords2D{
    WIN_WIDTH:Number,
    WIN_HEIGHT:Number,
    WIN_MAX:Number,
    MAX_VAL:Number,
    data_array:Vec<(u8,u8,u8,u8)>,
    pub transform:Transform2D,
    pub coord_system_color:(u8,u8,u8),
    pub coord_text_color:(u8,u8,u8),
    pub coord_text_size:f32,
}
impl Coords2D{
    pub fn new(
                win_width:Number,
                win_height:Number,
                max_val:Number,
                win_max:Number
                )->Coords2D{
        let mut transform = Transform2D::new(0.,0.,0.,0.,false,false);
        transform.set_scale(win_max/max_val);
    

        Coords2D{
            WIN_WIDTH:win_width,
            WIN_HEIGHT:win_height,
            WIN_MAX:win_max,
            MAX_VAL:max_val,
            data_array:Vec::new(),
            transform,
            coord_system_color:(255,255,255),
            coord_text_color:(150,150,150),
            coord_text_size:16.
        }
    }


    pub fn get_render_point(&mut self,point:(Number,Number))->(Number,Number){
        let origin = self.transform.get_transform(0.,0.);
        self.transform.get_transform(point.0,point.1)
    }
    
    pub fn scale(&mut self,factor:f32){
        let s = self.transform.get_scale();
        self.transform.set_scale(s*factor);
        self.MAX_VAL = self.WIN_MAX/self.transform.get_scale();
    }
    
    pub fn render_coord_system(&mut self, canvas:&mut sdl2::render::Canvas<sdl2::video::Window>){
        let small_line_col = (0,30,0);
        let coord_line_col = (0,255,0);
        
        let num_divs = 10i32;
        let max_f32 = self.MAX_VAL;
        let div_factor = max_f32/(num_divs as f32);
        
        // for coordinates
        let (X_xstart,X_ystart) = self.transform.get_transform(-self.MAX_VAL,0.);
        let (X_xend,X_yend) = self.transform.get_transform(self.MAX_VAL,0.);
        

        let (Y_xstart,Y_ystart) = self.transform.get_transform(0.,-self.MAX_VAL);
        let (Y_xend,Y_yend) = self.transform.get_transform(0.,self.MAX_VAL);
        


        (-num_divs..(num_divs+1)).step_by(1 as usize).for_each(|z|{
            let m = (z as f32)*div_factor;
            let (x,y) = self.transform.get_transform(m as Number,0.);   //x axis
            let (x_yaxis,y_yaxis) = self.transform.get_transform(0.,m as Number);   //y axis
            
            // x axis 
            rendering_system::render_text((x as i32,y as i32),canvas,m.to_string(),self.coord_text_size,self.coord_text_color);
           rendering_system::render_line(
               (x,Y_ystart),
               (x,Y_yend),
               small_line_col,
               canvas
               );



            //  y axis 
            rendering_system::render_text((x_yaxis as i32,y_yaxis as i32),canvas,m.to_string(),self.coord_text_size,self.coord_text_color);
            rendering_system::render_line(
                (X_xstart,y_yaxis),
               (X_xend,y_yaxis),
               small_line_col,
               canvas
               );
        });
        
        // render x coordinate
        rendering_system::render_line(
            (X_xstart,X_ystart),
            (X_xend,X_yend),
            coord_line_col,
            canvas
        );
        

        // render y coordinate
        rendering_system::render_line(
            (Y_xstart,Y_ystart),
            (Y_xend,Y_yend),
            coord_line_col,
            canvas
        );

    }

    pub fn render_rect(&mut self, point:(f32,f32),canvas:&mut sdl2::render::Canvas<sdl2::video::Window>){
        let (x,y) = self.transform.get_transform(point.0,point.1);
        rendering_system::render_rect(canvas,sdl2::rect::Rect::new(x as i32,y as i32,4,4),(0,0,200));
    }


    
    pub fn render_filled_rect_color(&mut self,point:(f32,f32),color:(u8,u8,u8),canvas:&mut sdl2::render::Canvas<sdl2::video::Window>){
        let (x,y) = self.transform.get_transform(point.0,point.1);
        rendering_system::render_rect(canvas,sdl2::rect::Rect::new(x as i32,y as i32,4,4),color );
    }



    


    // FIXME 
    // SDL BASED STUFF 
    pub fn render(&mut self,x:Number,y:Number){
    }
}





// Maybe add splines idk 




















