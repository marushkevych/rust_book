pub fn run() {
  let days = [
    ("first", "A partridge in a pear tree"),
    ("second", "Two turtle doves"),
    ("third", "Three French hens"),
    ("fourth", "Four calling birds"),
    ("fifth", "Five gold rings"),
    ("sixth", "Six geese a laying"),
    ("seventh", "Seven swans a swimming"),
    ("eighth", "Eight maids a milking"),
    ("ninth", "Nine drummers drumming"),
    ("tenth", "Ten pipers piping"),
    ("eleventh", "Eleven ladies dancing"),
    ("twelfth", "Twelve Lords a leaping")
  ];

  let mut gifts: Vec<&str> = vec![];

  for day in days.iter() {
    println!();
    let (n, gift) = day;
    gifts.push(gift);
    println!("On the {} day of Christmas my true love sent to me", n);
    for gift in gifts.iter().rev() {
      println!("{}", gift);
    }
  }
}