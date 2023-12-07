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
        use Leg::*;

        let won: bool = if self.playtype == other.playtype {
            // samefoot type
            matches!(
                (&self.leg_played, &other.leg_played),
                (Left, Left) | (Right, Right)
            )
        } else {
            matches!(
                (&self.leg_played, &other.leg_played),
                (Right, Left) | (Left, Right)
            )
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
