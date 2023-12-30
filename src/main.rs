fn main() {
    let carol_lines = [
        "drummers drumming",
        "pipers piping",
        "lords a-leaping",
        "ladies dancing",
        "maids a-milking",
        "swans a-swimming",
        "geese a-laying",
        "golden rings",
        "calling birds",
        "french hens",
        "turtle doves and",
        "partridge in a pear tree",
    ];
    let number_string = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "elevent", "twelfth",
    ];
    let count_strings = [
        "A", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven",
        "Twelve",
    ];

    for i in 0..carol_lines.len() {
        println!("\n[Verse {}] ", i + 1);
        println!(
            "On the {} day of Christmas, my true love sent to me ",
            number_string[i]
        );
        for j in (0..i + 1).rev() {
            println!(
                "{} {}",
                count_strings[j],
                carol_lines[carol_lines.len() - j - 1]
            );
        }
    }
}
