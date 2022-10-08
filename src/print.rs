pub fn run() {
    //Print to console
    println!("Hello from print.rs file");
    println!("Number: {}",1);

    //Basic formatting
    println!("{} is from {}","Brad","Mass");

    //Positional arguments
    println!("{0} is from {1} and {0} likes to {2}","Brad", "Mass", "code");

    //Named arguments
    println!("{name} likes to play {activity}",name="John",activity="Baseball");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug traits -- tuple
    println!("{:?}",(12, true, "hello"));

    //basic math
    println!("10 + 10 = {}", 10 + 10);
}