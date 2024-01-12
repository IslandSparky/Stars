//Stars  Program of dynamic art like a screen saver
/*MIT License
Copyright (c) 2023 Darwin Geiselbrecht
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use sdl2::rect::{Rect};
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::gfx::primitives::DrawRenderer;

use rand::Rng;

const X_MAX: u32= 2200;              // size of window in x direction
const Y_MAX: u32 = 1300;             // size of window in y direction
const NUM_STARS:usize = 200;         // number of each type of stars
const MIN_SPEED:f32 = 0.1;           // minimum speed of movement
const MAX_SPEED:f32 = 2.;            // maximum speed of movement
const MIN_SIZE:u32 = 30;             // minimum size of star
const MAX_SIZE:u32 = 100;            // maximum size of star
const MIN_POINTS:u32 = 3;            // minimum number of points
const MAX_POINTS:u32 = 8;            // maximum number of points
const MIN_DEPTH :f32 = 20.;          // minimum depth of notch in percent
const MAX_DEPTH:f32 = 70.;           // maximum depth of notch in percent

const PI: f32 = 3.1416;
const TWOPI: f32 = 6.2832;

#[derive(Clone,Copy,PartialEq)]
enum Rotation{
    CW,
    CCW
}

#[derive(Clone,Copy)]
struct Star {
    size: u32,
    notch_depth: f32,           // depth of notch between points in percent
    num_points: u32,
    x: f32,
    y: f32,
    speed: f32,
    direction: f32,
    outer_color: Color,
    inner_color: Color,
    outer_angle: f32,
    inner_angle: f32,
   
    rotation_speed: f32,
    rotation_direction:Rotation
}

impl Star {

    // move the star and check for collisions
    fn move_star(&mut self) {
        let half_size: u32 = self.size/2;
        self.x += self.speed * self.direction.cos() ;
        if self.x >= (X_MAX - half_size) as f32 {
            self.direction = PI- self.direction;
            self.x = (X_MAX - half_size) as f32 ;
        }    

        if self.x - (half_size as f32) <= 0.0 { 
            self.direction = PI - self.direction;
            self.x =  half_size as f32;
        }
        
        self.y += self.speed * self.direction.sin();
        if self.y >= (Y_MAX - half_size) as f32 { 
            self.direction = TWOPI - self.direction;
            self.y = (Y_MAX - half_size) as f32;
        }
        if self.y - (half_size as f32) <= 0.0 {
            self.direction = TWOPI - self.direction;
            self.y = half_size as f32;
        }
    }

    // re-draw the star, requires outer and inner draw
    fn update(&mut self,canvas:&mut Canvas<Window>){

      draw( canvas,self.x,self.y,self.size,self.num_points,
        self.notch_depth,self.outer_angle,self.outer_color);

      let radius: f32 = self.size as f32;               // now draw the inner star
      let inner_size: u32 =( radius - (radius * (self.notch_depth * 0.01)) ) as u32;    

      draw( canvas,self.x,self.y,inner_size,self.num_points,
        self.notch_depth,self.inner_angle,self.inner_color);

      match self.rotation_direction{
      Rotation::CW=> {
            self.outer_angle -= self.rotation_speed;
            self.outer_angle = fix_angle(self.outer_angle); 

            self.inner_angle += self.rotation_speed;
            self.inner_angle = fix_angle(self.inner_angle); 
        },
      Rotation::CCW => {
            self.outer_angle += self.rotation_speed;
            self.outer_angle = fix_angle(self.outer_angle); 

            self.inner_angle -= self.rotation_speed;
            self.inner_angle = fix_angle(self.inner_angle);             
        },
  
      }
    }
    // morph the star by randomly changing some of its characteristics (or not)
    fn morph(&mut self) {
        let mut rng = rand::thread_rng();
        // randcomly change the number of points 
        let mut guess: u8 = rng.gen_range(0,3);     
        match guess{
            0 => {},                                // do nothing
            1 => {
                if self.num_points < MAX_POINTS{
                        self.num_points += 1;       // randomly add a point
                }
            },
            2 => {
                if self.num_points > MIN_POINTS{
                    self.num_points -= 1;           // randomly remove a point                
                }                        
            },
            _ => {}
        }

        // randomly chnge the number of points
        guess= rng.gen_range(0,3);
        match guess{
            0 => {},                                // do nothing
            1 => {
                if self.size < MAX_SIZE - 10{
                        self.size += 10;       // randomly add a bit to size
                }
            },
            2 => {
                if self.size > MIN_SIZE + 10{
                    self.size -= 10;           // randomly remove a bit from size                
                }                        
            },
            _ => {}
        }

        //randomly change the notch depth
        guess= rng.gen_range(0,3);
        match guess{
            0 => {},                                // do nothing
            1 => {
                if self.notch_depth < MAX_DEPTH - 10.{
                        self.notch_depth += 10.;       // randomly add a bit to depth
                }
            },
            2 => {
                if self.notch_depth > MIN_DEPTH + 10.{
                    self.notch_depth -= 10.;           // randomly remove a bit from depth                
                }                        
            },
            _ => {}
        } 

        //randomly change the rotation direction
        guess= rng.gen_range(0,3);
        match guess{
            0 => {},                                // do nothing
            1 => {
                self.rotation_direction = Rotation::CW;
            },    
            2 => {
                self.rotation_direction = Rotation::CCW;
            },
            _ => {}
        } 

         //randomly change the colors (one at a time)
        guess= rng.gen_range(0,3);
        match guess{
            0 => {},                                // do nothing
            1 => {
                self.outer_color = random_color();
            },
            2 => {
                self.inner_color = random_color();                     
            },
            _ => {}
        }         
    }               

    //randomize starting position and direction   
    fn randomize(&mut self) {                               
        let mut rng = rand::thread_rng();
        self.x = rng.gen_range(0.0, X_MAX as f32);
        self.y = rng.gen_range(0.0 ,Y_MAX as f32);
       
        self.direction = rng.gen_range(0.,PI);
        self.speed = rng.gen_range(MIN_SPEED,MAX_SPEED);
        self.size = rng.gen_range(MIN_SIZE,MAX_SIZE) as u32;
        self.num_points = rng.gen_range(MIN_POINTS,MAX_POINTS) as u32;
        self.notch_depth = rng.gen_range(MIN_DEPTH,MAX_DEPTH);
        self.outer_color = random_color();
        self.inner_color = random_color();


        let guess: u8= rng.gen_range(0,2);              // randomize direction of rotation
        match guess{
            0=>{
                self.rotation_direction = Rotation::CW;
            },
            1=>{
                self.rotation_direction = Rotation::CCW;
            },
            _=> {}
        }
    }

} // end of the star implementation

// draw a star - this is called twice to make the imbedded stars   
fn draw(canvas:&mut Canvas<Window>,x:f32,y:f32,size:u32,num_points:u32,notch_depth:f32,angle:f32,
        color:Color){
    let radius: f32 = size as f32/2.;
    let inner_radius: f32 = radius - (radius * (notch_depth * 0.01));

    let mut x_start = x + (angle.cos()* radius);
    let mut y_start = y - (angle.sin()* radius); 


    for i_ in 1 .. num_points + 1{

      let next_angle:f32 = angle + (i_ as f32 *(TWOPI/(num_points as f32)));

      let x_end = x + ( (next_angle.cos()) * radius );  
      let y_end = y - ( (next_angle.sin()) * radius );  

      let mid_angle = next_angle - PI/num_points as f32;

      let x_mid = x + ( (mid_angle.cos()) * inner_radius );  
      let y_mid = y - ( (mid_angle.sin()) * inner_radius );  

      //canvas.draw_line((x_start as i32,y_start as i32),(x_mid as i32,y_mid as i32)).unwrap();
      canvas.filled_trigon(x_start as i16,y_start as i16,x_mid as i16,y_mid as i16,
       x as i16,y as i16,color).unwrap();
      canvas.draw_line((x_mid as i32,y_mid as i32),(x_end as i32,y_end as i32)).unwrap();

      x_start = x_end;
      y_start = y_end;

    }   
}   

// check to see if any two stars collided and, if so, bounce and morph them
fn check_collisions(stars:&mut Vec<Star>) {


    for j in 0 .. stars.len() {

        for k in 0 .. stars.len() {
            if j != k {

                let distance = find_distance_between_stars(stars[j],stars[k]) as f32;
                let min_distance = (stars[j].size/2 + stars[k].size/2 ) as f32;
                if distance <= min_distance { 
                    stars[j].direction = bounce(stars[j],stars[k]);
                    stars[j].morph();
                }
            }

        }
    }
}
//fix angle, keep radian angles in the range of 0 to 2pi
fn fix_angle(angle:f32)->f32{
    let mut _angle = angle;
    if _angle > TWOPI {_angle -= TWOPI}
    else if _angle < 0. {_angle += TWOPI}
    _angle
}


// find the distance in pixels between two sets of x,y coordinates
fn find_distance (x1:f32,y1:f32,x2:f32,y2:f32) -> f32 {
    let xdist = x2 - x1;
    let ydist = y2 - y1;
    ( (xdist * xdist) + (ydist * ydist) ).sqrt()
}
// find the distance between two stars
fn find_distance_between_stars (star_1:Star,star_2: Star) -> f32 {
    find_distance(star_1.x,star_1.y,star_2.x,star_2.y)
}

// find the direction in radians between two sets of x,y coordinates
fn find_direction (x1:f32,y1:f32,x2:f32,y2:f32) -> f32 {
    let xdist = x2 - x1;
    let ydist = y2 - y1;
    ydist.atan2(xdist)
}

// find direction between stars
fn find_direction_between_stars (  star_1:Star, star_2: Star) -> f32 {
    find_direction(star_1.x,star_1.y,star_2.x,star_2.y)
}

// bounce - cause a star to bounce off another, returns the desired direction of star_1
fn bounce (star_1:Star,star_2:Star) -> f32 {
    let direction = find_direction_between_stars(star_1,star_2) - PI;
    fix_angle(direction)

}

// randomly return a saturated color, including black and white
fn random_color() -> Color {

    let mut rng = rand::thread_rng();
    let mut color:Color = Color::BLACK;

    let guess: u8 = rng.gen_range(0,5);
    match guess {
        0 => {color = Color::BLACK},
        1 => {color = Color::WHITE},
        2 => {color = Color::RED},
        3 => {color = Color::GREEN},
        4 => {color = Color::BLUE},
        _ => {}                
    }
    color                                   // return color     
}


fn main() -> Result<(), String> {


    let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Stars", X_MAX, Y_MAX)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().present_vsync().build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    

    let mut stars = Vec::with_capacity(NUM_STARS);          // set up the star vector and push in some stars

    for _i in 0 ..NUM_STARS {
       stars.push(Star {size:100,num_points:5,notch_depth: 40.,x:(X_MAX/2) as f32,y:(Y_MAX/2) as f32,
        speed:0.,direction:0.,outer_color:Color::WHITE,inner_color:Color::BLACK,
        outer_angle:0.,inner_angle:0.,rotation_speed:0.05,rotation_direction:Rotation::CCW});

     }


    for  i in 0 .. stars.len(){                    // Randomize starting location and direction
        stars[i].randomize();                           
    }


    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        
        canvas.set_draw_color(Color::GRAY);  
        canvas.clear();

        for  i in 0 .. stars.len(){                                 // Update all the stars

            stars[i].move_star();
            stars[i].update(&mut canvas);
        };


        check_collisions(&mut stars);

        canvas.present();

        // Time management! not needed, synced to vsync
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    } //running loop

    Ok(())
    
}