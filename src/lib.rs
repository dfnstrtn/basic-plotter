/*
 * Cong Y.Z., & Ponnambalam S.G., (2009). Mobile 
    robot path planning using ant colony optimization.
    IEEE International Conference on Advanced 
    Intelligent Mechatronics, pp. 851-856
*/
use std::thread::sleep_ms;
pub const edge_length:f32 = 2.0;
pub struct ACOnode{
    pheromone:f32,
    visited:bool,
    resistance:f32
}



pub struct ACOgrid{
    width:usize,
    height:usize,
    data:Vec<Vec<ACOnode>>,
    evap_rate:f32,
    alpha:f32,
    beta:f32,
    n_ants:usize,
    best_path:Vec<(usize,usize)>,
    best_path_cost:f32
}

impl ACOgrid{
    pub fn new(width:usize,height:usize,pheromone:f32)->ACOgrid{
        ACOgrid{
            evap_rate:0.0005,
            width,
            height,
            data:(0..height).map(|_|{
                (0..width).map(|_|ACOnode{
                    pheromone,
                    visited:false,
                    resistance:1.0
                }).collect::<Vec<ACOnode>>()
            }).collect::<Vec<Vec<ACOnode>>>(),
            alpha:2.0,
            beta:-0.5,
            n_ants:10,
            best_path:Vec::new(),
            best_path_cost:f32::INFINITY
        }
    }

    pub fn visit(&mut self,pos:(usize,usize)){
        self.data[pos.0][pos.1].visited = true;
        self.data[pos.0][pos.1].resistance = 25.0;
    }

    pub fn add_pheromone(&mut self, pos:(usize,usize),pheromone:f32){
        self.data[pos.0][pos.1].pheromone +=pheromone; 
    }

    pub fn at(&self,y:isize,x:isize)->Option<ACOnode>{
        if(x<self.width as isize)&&(y<self.height as isize )&&(x>=0)&&(y>=0){
            let pheromone = self.data[y as usize][x as usize].pheromone;
            let visited = self.data[y as usize ][x as usize].visited;
            let resistance = self.data[y as usize ][x as usize].resistance;
            return Some(ACOnode{
                pheromone,
                visited,
                resistance
            })
        }else{
            None
        }
    }


    // Create a color keying system for this
    //  obstacles are green 
    //  pheromone gets progressively redder
    //  paths are white 
    //  unexplored paths are black 
    pub fn display(&self){
        self.data.iter().for_each(|m|{
            m.iter().for_each(|n|{
                if n.resistance>500.0{
                    print!("\x1b[38;2;1;200;4m[1]\x1b[0m");
                }else{
                    if n.visited{
                        print!("\x1b[38;2;1;1;200m[0]\x1b[0m");
                    }else{
                        if(n.pheromone>0.04){
                            print!("\x1b[38;2;{};1;1m[0]\x1b[0m",((n.pheromone*500.0).abs() as i32)%256);
                        }else{
                            print!("\x1b[38;2;1;1;1m[0]\x1b[0m");
                        }
                    }
                }
            });
            println!();
        })
    }
    
    pub fn iterate(&mut self){
        //  find_next_node()
        //  add_node_to_array()
        //  update_path_sum()
        //  update_best_path()
        //  evaporate()
    }

    pub fn test_iterate(&mut self){
        let mut ant_paths :Vec<(Vec<(usize,usize)>,f32)>=Vec::new();
        let mut best_path:Vec<(usize,usize)>=Vec::new();
        let mut best_path_length :f32 = 1000000.0;
        let end_pos = (17,17);
        self.add_pheromone(end_pos, 1.0);
        
        (0..self.n_ants).for_each(|ant|{
            //println!("ant {}",ant);
            let mut ant_pos = (0,0);
            let mut path_len=0.0;
            let mut path:Vec<(usize,usize)> = Vec::new();
            while ant_pos!=end_pos{
                self.visit(ant_pos);
                if let Some(v) = self.find_next_node(ant_pos){
                    path.push(ant_pos);
                    ant_pos = (v.0,v.1);
                    path_len+=v.2;
                }else{
                    //self.display();
                    path.clear();
                    //println!("Stuck!");
                    break;
                }
                
            }
            if (!path.is_empty()){
                ant_paths.push((path.clone(),path_len));
                if path_len<self.best_path_cost{
                    
                    self.best_path_cost = path_len;
                    self.best_path = path;
                    //println!("{:?}",self.best_path);
                }
            }
            self.unmark_all_nodes();
        });

        for path in ant_paths{
            let pheromone = 1.0/path.1;
            path.0.iter().for_each(|m|{
                self.add_pheromone((m.0,m.1), 10.0*pheromone)
            })
        }
       
        self.evaporate();
    }



    // I completely reject any possibility of going into any occupied cell here
    // but under other circumstances we could model a robot travelling through a potential field 
    pub fn get_distance(&self, start:(usize,usize), end:(usize,usize))->Option<f32>{
        let t = ((start.0 as f32 -end.0 as f32).powi(2)  + (start.1 as f32 - end.1 as f32).powi(2)).sqrt();
        if let Some(v)=self.at(end.0 as isize,end.1 as isize){
                if v.resistance>500.0{
                    return None;
                }else{
                    return Some(t);
                }
        }else{
            return None
        }
    }


    // TODO : add a negativity factor
    // start(Y,X)
    // checks for transitions to the nearest 8 nodes's sum 
    pub fn find_total_transitions_sum(&self, start:(usize,usize))->Option<f32>{
        let start_y = start.0  as isize - 1;
        let start_x = start.1 as isize - 1;
        let mut trans_sum=0.0;
        (0..3).for_each(|m|{
            (0..3).for_each(|n|{
                if let Some(node) = self.at(start_y+m ,start_x+n){
                    if !node.visited{
                        if let Some(distance) = self.get_distance(start,((start_y+m) as usize,(start_x+n) as usize)){
                            let transition_prob = (node.pheromone).powf(self.alpha) * (distance).powf(self.beta);
                            trans_sum+=transition_prob;
                        }
                    }    
                }
            })
        });
        return Some(trans_sum);
    }


    pub fn roulette_selection(moves:&Vec<(usize,usize,f32)>)->Option<usize>{
        let random_variable = rand::random::<f32>();
        let mut probability=0.0;
        let mut index=0;
        for m in moves{
            probability+=m.2;
            if probability>random_variable{
                return Some(index);
            }
            index+=1;
        }
        None
    }


    // start (Y,X)
    #[deprecated(since="0.0.0", note="please kill yourself")]
    pub fn _transition_probability(&self, start:(usize,usize),end:(usize,usize))->Option<f32>{
        let node_end = if let Some(v) = self.at(end.0 as isize, end.1 as isize){
            v
        }else{
            return None
        };

        let node_start = if let Some(v) = self.at(start.0 as isize, start.1 as isize){
            v
        }else{
            return None
        };

        if let Some(distance) = self.get_distance(start,end){
            let transition_prob = (node_end.pheromone).powf(self.alpha) * (distance).powf(self.beta);
            let trans_prob_total = if let Some(v) = self.find_total_transitions_sum(start){
                v
            }else{
                return None
            };
            return Some(transition_prob/trans_prob_total);
        }else{
            return None
        }
    }


    pub fn find_next_node(&mut self,current:(usize,usize))->Option<(usize,usize,f32)>{
        let start_y = current.0  as isize - 1;
        let start_x = current.1 as isize - 1;
        let trans_prob_total = if let Some(v) = self.find_total_transitions_sum(current){
            v
        }else{
            return None
        };
        let mut next_moves:Vec<(usize,usize,f32)> = Vec::new();
        (0..3).for_each(|m|{
            (0..3).for_each(|n|{
                if let Some(node) = self.at(start_y+m,start_x+n){
                    if !node.visited{
                        if let Some(distance) = self.get_distance(current,((start_y+m ) as usize,(start_x+n) as usize)){
                            let transition_prob = node.pheromone.powf(self.alpha) * distance.powf(self.beta)/trans_prob_total;
                            next_moves.push(((start_y+m) as usize ,(start_x+n) as usize,transition_prob));
                        }
                    }
                }
            })
        });

        if let Some(index) = Self::roulette_selection(&next_moves){
            let node = next_moves[index].clone();
            let distance = self.get_distance(current,(node.0,node.1)).unwrap();
            return Some((node.0,node.1,distance));
        }else{
            None
        }
    }


    pub fn unmark_all_nodes(&mut self){
        self.data.iter_mut().for_each(|m|{
            m.iter_mut().for_each(|n|{
                n.visited = false;
            })
        })
    }

    pub fn evaporate(&mut self){
        let evap_rate  = self.evap_rate;
        self.data.iter_mut().for_each(|m|{
            m.iter_mut().for_each(|n|{
                n.pheromone-= evap_rate;
            })
        })        
    }
}

pub fn test_aco(){
    let mut new_aco = ACOgrid::new(40,40,0.01);
    (0..6).for_each(|m|{
        new_aco.data[6][m].resistance = 1000.0;
    });
    (0..30).for_each(|m|{
        new_aco.data[m][9].resistance = 1000.0;
    });
    (3..15).for_each(|m|{
        new_aco.data[12][m].resistance = 1000.0;
    });

    //(7..15).for_each(|m|{
      //  new_aco.data[m][9].resistance = 1000.0;
    //});

    println!("TEST");
    new_aco.display();
    (0..5000).for_each(|m|{
        new_aco.test_iterate();
        //for m in &mut new_aco.best_path{
            //print!("m ->");
          //  new_aco.data[m.0][m.1].visited=true;
        //}
        //new_aco.display();
        //new_aco.unmark_all_nodes();
    }
    );

    println!("BEST PATH");
    new_aco.unmark_all_nodes();
    
    for m in &mut new_aco.best_path{
        //print!("m ->");
        new_aco.data[m.0][m.1].visited=true;
    }
    new_aco.display();
}