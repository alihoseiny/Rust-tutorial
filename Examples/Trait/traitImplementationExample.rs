trait Fly {
        fn fly(&self);
            fn land(&self);
}

struct Kaftar ();

impl Fly for Kaftar {
        fn fly(&self) {
            println!("Kafter The Kakol Be Sar is flying");
        }

       fn land(&self) {
           println!("Kafter The Kakol Be Sar is landing");
       }

}

fn main() {
    let kakol_be_sar = Kaftar{};
    kakol_be_sar.fly();
    kakol_be_sar.land();
}
