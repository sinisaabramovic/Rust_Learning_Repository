pub fn run() {

    let mut iterrator = 1;

    while iterrator < 1000 {

        iterrator *= 2;

        if iterrator == 64 {continue;}
        // if iterrator == 256 {break;}

        println!("iterrator = {}", iterrator);
    }

    let mut second_iterrator = 1;
    loop {

        second_iterrator *= 2;
        println!("second_iterrator = {}", second_iterrator);

        if second_iterrator == 1<<10 {break;}
    } // while true

}