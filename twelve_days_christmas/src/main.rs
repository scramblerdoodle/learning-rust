const ORDINAL: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const GIFTS: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five gold rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];
fn main() {
    for day in 1..=12 {
        // Same carol, change day
        let nth_day_carol: String = format!(
            "On the {} day of Christmas, my true love sent to me",
            ORDINAL[day - 1]
        );
        println!("{nth_day_carol}");

        for d in (2..=day).rev() {
            println!("{},", GIFTS[d - 1]);
        }

        // Print last part of carol,
        // if not first day, add the "And" and convert to lowercase
        let fst_day: String = if day == 1 {
            format!("{}", GIFTS[0])
        } else {
            format!("And {}", GIFTS[0].to_lowercase())
        };
        println!("{fst_day}");

        // Add line break between verses
        println!("");
    }
}
