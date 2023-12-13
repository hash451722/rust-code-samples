

const FT2M: f64 = 0.3048;

#[derive(Debug)]
struct Vector2d {
    x: f64,
    y: f64,
}

impl Vector2d {
    fn length(&self) -> f64 {
        (self.x*self.x + self.y*self.y).sqrt()
    }
    fn normalize(&self) -> Vector2d {
        let l: f64 = self.length();
        Vector2d {x: self.x / l, y: self.y / l}
    }

    fn add(&self, s: &Vector2d) -> Vector2d{
        Vector2d { x: self.x+s.x, y: self.y+s.y }
    }
    fn subtract(&self, s: &Vector2d) -> Vector2d{
        Vector2d { x: self.x-s.x, y: self.y-s.y }
    }


}


#[derive(Debug)]
struct Stone {
    id: u8,
    // color: String,
    pos: Vector2d,
    velocity: Vector2d,
    rot: f64,  // rotation angle
    omega: f64,  // angular velocity
    inplay: bool,
    mass: f64,
    radius: f64,
}

impl Stone {
    fn new(id: u8) -> Stone {
        Stone {
            id: id,
            // color: "red",
            pos: Vector2d { x: 0., y: 0. },
            velocity: Vector2d { x: 0., y: 0. },
            rot: 0.,
            omega: 0.,
            inplay: false,
            mass: 20.,
            radius: 0.2,
        }
    }

    fn distance(&self, s: &Stone) -> f64 {
        ((s.pos.x-self.pos.x).powi(2) + (s.pos.y-self.pos.y).powi(2)).sqrt()
        - (self.radius + s.radius)
    }

    fn collide(&self, s: &Stone) -> bool {
        let ret: bool;
        if self.inplay == true && s.inplay == true  {
            if self.distance(s) < 0.0{
                ret = true;
            } else {
                ret = false;
            }
        } else {
            ret = false;
        }
        ret
    }

    fn interaction(&self, s: &Stone) -> f64 {
        let normal_vec: Vector2d = s.pos.subtract(&self.pos).normalize();
        let tangential_vec: Vector2d = Vector2d { x: normal_vec.y, y: -normal_vec.x };
        

        

        0.0
    }

}


fn init_stones() -> [Stone; 16] {
    return [
        Stone::new(0),
        Stone::new(1),
        Stone::new(2),
        Stone::new(3),
        Stone::new(4),
        Stone::new(5),
        Stone::new(6),
        Stone::new(7),
        Stone::new(8),
        Stone::new(9),
        Stone::new(10),
        Stone::new(11),
        Stone::new(12),
        Stone::new(13),
        Stone::new(14),
        Stone::new(15),
    ];
}



#[derive(Debug)]
struct Sheet {
    tee: Vector2d,
    width: f64,
}

impl Sheet {
    fn new() -> Sheet{
        Sheet {
            tee: Vector2d { x: 0.0, y: (6.+6.+114.)*FT2M },
            width: 4.750,
        }
    }
}




fn main() {
    let vec1: Vector2d = Vector2d {x: 1., y: 1.};
    println!("rect1 is {:#?}", vec1);
    println!("Length {}: ", vec1.length());
    println!("Normalized {:?}: ", vec1.normalize());



    let mut stones: [Stone; 16] = init_stones();

    println!("{:#?}", stones[0]);
    stones[0].inplay = true;
    println!("{:#?}", stones[0]);

    println!("Distance: {:?}", stones[0].distance(&stones[1]));
    println!("Collide: {:?}", stones[0].collide(&stones[1]));



}
