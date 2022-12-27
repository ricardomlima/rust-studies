/**
 *
 * The if let syntax lets you combine if and let into a less
 * verbose way to handle values that match one pattern while
 * ignoring the rest.
 *
 */

#[derive(Debug)]
enum HouseVariant {
    TechHouse,
    Minimal,
    Deep,
}

enum ElectronicMusic {
    Techno,
    Trance,
    House(HouseVariant),
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The max config value {}", max),
        _ => (),
    }

    // Instead we can do this
    // Just syntax sugar
    // loses exhaustive checking that match enforces
    if let Some(max) = config_max {
        println!("The max config value {}", max);
    }

    let preferred_music = ElectronicMusic::House(HouseVariant::Deep);

    if let ElectronicMusic::House(house_type) = preferred_music {
        println!("You like {:?} House", house_type)
    } else {
        println!("I don't like the music you like");
    }
}