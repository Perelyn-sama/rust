fn main() {
    // In general, the `{}` will be automatically replaced with any 
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becommes an 132. You can change what type 31 is 
    // by providing a suffix. The number 31i64 for example has the type i64.

    // There are various optional patterns this works with. Positional
    // arguements can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguements.
    println!("{subject} {")

}