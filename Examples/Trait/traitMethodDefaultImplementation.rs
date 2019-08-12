trait Fly {
    fn fly(&self);
    fn land(&self) {
        println!("Flyable Object now landing.");
    }
}

struct Kaftar ();
struct AirPlane();
struct UnknownFlyable();

impl Fly for UnknownFlyable {
    fn fly(&self) {
        println!("Unknown flyable is flying.");
    }
}

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

fn main() {
    let airplane = AirPlane{};
    let kakol_be_sar = Kaftar{};
    let unknown = UnknownFlyable{};

    airplane.land();
    kakol_be_sar.land();
    unknown.land();
}
