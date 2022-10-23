fn main() {
    for day in 1..13 {
        day_start(day);
        for gift in (1..(day + 1)).rev() {
            gift_verse(gift, if gift == 1 && day != 1 { "and " } else { "" })
        }
    }
}

fn day_start(n: u32) {
    let day = match n {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    };
    println!("\nOn the {} of Christmas \nmy true love sent to me.", day);
}

fn gift_verse(n: u32, prefix: &str) {
    let gift_text = match n {
        1 => "a Partridge in a Pear Tree",
        2 => "Two Turtle Doves",
        3 => "Three French Hens",
        4 => "Four Calling Doves",
        5 => "Fiven Golden Rings",
        6 => "Six Gesse a Laying",
        7 => "Seven Swans a Swimming",
        8 => "Eight Maids a Milking",
        9 => "Nine Ladies Dancing",
        10 => "Ten Lords a Leaping",
        11 => "Eleven Pipers Piping",
        12 => "12 Drummers Drumming",
        _ => "",
    };
    println!("{}{}", prefix, gift_text);
}
