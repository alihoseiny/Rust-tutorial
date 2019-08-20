trait Fly {
    fn new(distance: u8) -> Self;
   fn fly(self: &Self);
   fn land(&self) {
       println!("Flyable Object now landing.");
   }
   fn change_distance(&mut self, distance: u8);
}

struct Kaftar {
    fly_distance: u8
}

impl Fly for Kaftar {
    fn new(distance: u8) -> Self {
        Kaftar {
            fly_distance: distance
        }
    }

    fn fly(&self) {
        println!("Kafter The Kakol Be Sar is flying");
    }

fn change_distance(& mut self, distance: u8) {
    self.fly_distance = distance;
                                                                                        }
}


fn main() {
    let mut kakol_be_sar = Kaftar::new(10);
    Kaftar::fly(&kakol_be_sar);
    Kaftar::change_distance(&mut kakol_be_sar, 100);
    println!("Flying distance: {}", kakol_be_sar.fly_distance);
}
