#[derive(PartialEq, Eq)]
enum PlayerType {
    SameFoot,
    DifferentFoot,
}

#[derive(PartialEq, Eq)]
enum Leg {
    Left,
    Right,
}

#[derive(PartialEq, Eq)]
struct AmpePlayer {
    playtype: PlayerType,
    leg_played: Leg,
}

impl AmpePlayer {
    pub fn play(&self, other: &AmpePlayer) -> bool {
        let won: bool = if self.playtype == other.playtype {
            // samefoot type
            match (&self.leg_played, &other.leg_played) {
                (Leg::Left, Leg::Left) => true,
                (Leg::Right, Leg::Left) => false,
                (Leg::Left, Leg::Right) => false,
                (Leg::Right, Leg::Right) => true,
            }
        } else {
            match (&self.leg_played, &other.leg_played) {
                (Leg::Left, Leg::Left) => false,
                (Leg::Right, Leg::Left) => true,
                (Leg::Left, Leg::Right) => true,
                (Leg::Right, Leg::Right) => false,
            }
        };

        won
    }
}
fn main() {
    let user = AmpePlayer {
        playtype: PlayerType::DifferentFoot,
        leg_played: Leg::Left,
    };

    let com = AmpePlayer {
        playtype: PlayerType::SameFoot,
        leg_played: Leg::Right,
    };

    let did_user_win = user.play(&com);

    if did_user_win {
        println!("We won!");
    }
}
