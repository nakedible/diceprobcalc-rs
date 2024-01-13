use diceprobcalc::*;

fn main() {
    // println!();
    // call_of_cthulhu(72, CallOfCthulhuDifficulty::Regular).print();
    // println!();
    // call_of_cthulhu_pushed(72, CallOfCthulhuDifficulty::Regular).print();
    // println!();
    // world_of_darkness(7, 8).print();
    // println!();
    // fate(3, 2).print();
    // println!();
    // runequest(29, 0).print();
    // println!();
    // runequest(30, 0).print();
    // println!();
    // runequest(31, 0).print();
    // print_range(1..=122, |s| runequest(s, 0))
    //rolemaster(50, RolemasterDifficulty::Medium.into()).print();
    //gurps(10, 10).print();
    alien_rpg::skill_check(2, 2, 0).print();
    println!();
    alien_rpg::skill_check_pushed(2, 2, 0).print();
}
