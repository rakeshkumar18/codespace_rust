// Enums
enum Conveyance{
    Car = 15,
    Train = 20,
    Air = 30,
}

impl Conveyance {
    fn travel_allowance(&self,  miles : i32) -> f32 {

        let allowance = match self {
            Conveyance::Car => miles as f32 * 14.0 * 2.0,
            Conveyance::Train => miles as f32 * 18.0 * 2.0,
            Conveyance::Air => miles as f32 * 30.0 * 2.0
        };

        allowance
    }
}

fn main() {
    let participant_1 = Conveyance::Car;
    let participant_2 = Conveyance::Train;
    let participant_3 = Conveyance::Air;

    println!("The participant 1 has a travel allownace of {}", participant_1.travel_allowance(60));
    println!("The prticipant 2 has a travel allowance of {}", participant_2.travel_allowance(120));
    println!("The participant 3 has a travel allowance of {}", participant_3.travel_allowance(60));

}
