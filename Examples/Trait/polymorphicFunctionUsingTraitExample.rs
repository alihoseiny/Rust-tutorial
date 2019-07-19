trait Fly {
    fn fly(&self);
    fn land(&self);
}

struct Kaftar ();
struct AirPlane();

impl Fly for Kaftar {
    fn fly(&self) {
        println!("Kafter The Kakol Be Sar is flying");
    }

   fn land(&self) {
        println!("Kafter The Kakol Be Sar is landing");
   }

}

impl Fly for AirPlane {
    fn fly(&self) {
        println!("Airplane is flying.");
    }

   fn land(&self) {
        println!("Airplane is landing.");
   }
}

fn fly_bird(flyable: &Fly) {
    flyable.fly();
}

fn main() {
    let airplane = AirPlane{};
    let kakol_be_sar = Kaftar{};
    fly_bird(&airplane);
    fly_bird(&kakol_be_sar);
}
