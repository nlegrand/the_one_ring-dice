use my_one_ring::dice as MorDice;

use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

    /// How many success dice
    #[arg(short, long, value_name = "DICE")]
    success_dice: u8,

    /// The roll is favoured
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    favoured: bool,

    /// The roll is ill-favoured
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    ill_favoured: bool,

    /// The character is weary
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    weary: bool,

    /// The character is miserable
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    miserable: bool,

}


fn main() {
    let feat_dice_result = MorDice::feat_dice();
    let success_dice_result = MorDice::success_dice();
    let favoured_feat_dice_result = MorDice::favoured_feat_dice();
    let ill_favoured_feat_dice_result = MorDice::ill_favoured_feat_dice();
    println!("Success dice: {}", success_dice_result);
    println!("Feat dice:{}", feat_dice_result);
    println!("Favoured feat dice:{}", favoured_feat_dice_result);
    println!("Ill favoured feat dice:{}", ill_favoured_feat_dice_result);
    let dp = MorDice::DicePool {
	feat: MorDice::Feat::Normal,
	success_dice: 3,
    };
    let outcome = dp.roll();
    println!("my outcome: {:?}", outcome);
    let dpf = MorDice::DicePool {
	feat: MorDice::Feat::Favoured,
	success_dice: 2,
    };
    let computed_result = outcome.compute(MorDice::WEARY_AND_MISERABLE);
    println!("my computed result: {:?}", computed_result);
    let favoured_outcome = dpf.roll();
    println!("my favoured outcome: {:?}", favoured_outcome);

    let cli = Cli::parse();

    let sd = cli.success_dice;

    let dp2 = MorDice::DicePool {
	feat: MorDice::Feat::Normal,
	success_dice: sd,
    };
    let outcome2 = dp2.roll();
    println!("my outcome2: {:?}", outcome2);
}
