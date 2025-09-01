fn main() {
    // we are doing "The twelve days of Christmas"
    // an array to hold all the days i.e first, second and all the gifts from the song
    let gifts = [
        "And a partridge in a pear tree!",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
        ];
    let days = [
        "first","second","third","fourth","fifth","sixth","seventh",
        "eighth","ninth","tenth","eleventh","twelfth"
    ];
    // a for loop for 0 to 12
    for day in 0..12{
        println!("On the {} day of christmas my true love sent to me ", days[day]);
        // another for loop for gift on that particular day, it will we in reverse
        for gift in (0..=day).rev(){
            if gift == 0 && day !=0{
                println!(" and {}", gifts[gift]);
            } else {
                println!("{}", gifts[gift]);
            }
        }
        println!();
    }
}
