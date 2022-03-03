//reading coord data from a file
use std::fs::File;
use std::io::Read;
fn read_file_data()->Result<String,()>{

    let mut data:String= String::new();
    if let Ok(mut s) = File::open("data.txt"){
        s.read_to_string(&mut data);
    };

    data.lines().for_each(|m|{
       let mut a =  m.split("|");
       if let Some(p) = a.next(){
           if let Ok(z) = p.parse::<f32>(){
                println!("{:?}",z);
           }
       }
    });


    data.lines().for_each(|m|{
        println!("{:?}",m);
    });

    Ok(data)
}



fn load_WS_to_array(array:&mut Vec<f32>, text:&mut String)->Result<(),()>{
    let txt = text.clone();
    let mut a = txt.split_once(':');
    if let Some(p)=a{
        if p.0=="WS"{
            array.clear();
            p.1.split("|").for_each(|m|{
                if let Ok(f) = m.parse::<f32>(){
                    array.push(f)
                }
            });
            return Ok(())
        }else{
            return Err(())
        }
    }else{
        return Err(())
    }
}



fn load_US_to_array(array:&mut Vec<f32>, text:&mut String){
    let mut a = text.split_once(':');
    if let Some(p)=a{
        //if p.0=="US"{ 
            array.clear();
            p.1.split("|").for_each(|m|{
                if let Ok(f) = m.parse::<f32>(){
                    array.push(f)
                }
            });
        //}
    }else{
        return 
    }
}



fn load_LS_to_array(array:&mut Vec<f32>, text:&mut String){
    array.clear();
    let mut a = text.split_once(':');
    if let Some(p)=a{
        p.1.split("|").for_each(|m|{
            if let Ok(f) = m.parse::<f32>(){
                array.push(f)
            }
        });
    }else{
        text.split("|").for_each(|m|{
            if let Ok(f) = m.parse::<f32>(){
                array.push(f)
            }
        }); 
    }
}

//FIXME
// HORRIBLY UNSAFE
fn load_XYTs_to_array(array:&mut Vec<f32>, text:&mut String){
    array.clear();
    let mut a = text.split("|");
    let x = a.next().unwrap().parse::<f32>().unwrap();
    let y = a.next().unwrap().parse::<f32>().unwrap();
    let z = a.next().unwrap().parse::<f32>().unwrap();
    array.push(x);
    array.push(y);
    array.push(z);
}




fn loadXYs_to_array(array:&mut Vec<f32>,text:&mut String){
    array.clear();
    let mut a = text.split("|");
    let x = a.next().unwrap().parse::<f32>().unwrap();
    let y = a.next().unwrap().parse::<f32>().unwrap();
    array.push(x);
    array.push(y);
}






fn loadSensorData(text:&mut String)->Vec<f32>{
    let text_data = text.split("|");
    text_data.map(|m|{
        m.parse::<f32>().unwrap()
    }).collect::<Vec<f32>>()
}




const M_CM:f32 = 1./100.;
const P_180:f32 = std::f32::consts::PI/180.; 
pub fn testSensorData()->std::io::Result<Vec<Vec<(f32,f32)>>>{
    let mut readings_sensor = std::fs::File::open("raw_data/lidar_data.txt")?;
    let mut reader_sensor = std::io::BufReader::new(readings_sensor); 
    let mut lines = reader_sensor.lines();
    let mut x_y_data = Vec::<Vec<(f32,f32)>>::new();
    let mut index=0.;
    while let Some(v) = lines.next(){
        if let Ok(mut m) = v{ 
            let d = loadSensorData(&mut m);
            x_y_data.push(
                d.iter().map(|z|{
                    let r = M_CM*z;
                    let x = (r*(index*P_180).cos() , r*(index*P_180).sin());  
                    index+=2.;
                    print!("[r{} theta{}]",r,index*P_180);
                    x
                }).collect::<Vec<(f32,f32)>>()
            )
        }

        println!();
    }
    Ok(x_y_data)
}




























use std::io::BufRead;
use std::io::Write;
pub fn test()->std::io::Result<Vec<(f32,f32,f32)>>{
    let mut readings = std::fs::File::open("raw_data/test8.txt")?;
    let mut readings_us = std::fs::File::open("raw_data/test_us8.txt")?;
    let mut readings_x_y_t = std::fs::File::open("raw_data/dead_obj_coord.txt")?;


    let mut file_data = String::new();
    let mut file_data_us = String::new();




    let mut reader = std::io::BufReader::new(readings);
    let mut reader_us = std::io::BufReader::new(readings_us);
    let mut reader_x_y_t = std::io::BufReader::new(readings_x_y_t);


    let mut data_anal:Vec<f32>=Vec::new();

    let mut ws_data:Vec<f32>=Vec::new();
    let mut us_data:Vec<f32>=Vec::new();
    let mut laser_data:Vec<f32>=Vec::new();
    let mut x_y_t_raw:Vec<f32> = Vec::new();


    let mut lines = reader_x_y_t.lines();
    
    let mut odometry_data = Vec::<(f32,f32)>::new();
    let mut x_y_z_data = Vec::<(f32,f32,f32)>::new();

    while let Some(v) = lines.next(){
        if let Ok(mut m) = v{
            
            //if let Ok(())=load_WS_to_array(&mut ws_data, &mut m){
              //  odometry_data.push((ws_data[0],ws_data[1]))
            //}
        
            load_XYTs_to_array(&mut x_y_t_raw, &mut m);
            x_y_z_data.push((x_y_t_raw[0],x_y_t_raw[1],x_y_t_raw[2])); 
        }
        x_y_t_raw.clear();
    } 
    Ok(x_y_z_data)
}












pub fn test2()->std::io::Result<Vec<(f32,f32)>>{
    let mut readings_x_y_t = std::fs::File::open("raw_data/x_y_values.txt")?;
    let mut reader_x_y_t = std::io::BufReader::new(readings_x_y_t);
    let mut x_ys_raw:Vec<f32> = Vec::new();
    let mut lines = reader_x_y_t.lines();
    let mut x_y_data = Vec::<(f32,f32)>::new();

    while let Some(v) = lines.next(){
        if let Ok(mut m) = v{ 
            loadXYs_to_array(&mut x_ys_raw, &mut m);
            x_y_data.push((x_ys_raw[0],x_ys_raw[1])); 
        }
        x_ys_raw.clear();
    } 
    Ok(x_y_data)
}








