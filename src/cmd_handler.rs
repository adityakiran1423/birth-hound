use std::env::{args, Args};

pub fn args_handler() {
    let hound_args: Args = args();
    println!("{:?}", hound_args);
    let args_count: usize = hound_args.len();
    //let hound_arg: Args = hound_args[3];
    
    if args_count == 2 {
        println!("write that switch case here");
        // write in another rs file code to handle the daemon commands
        // start : includes code to start the daemon
        // stop : stops the daemon/kills the program/exits the program
        // config : sets how many times to check csv in a day (default:3)
        // ~ options are 1, 2, 3
        // add : adds new dates to the csv
        // ~ handle the csv, when new date added, refresh or restart daemon
        
    }
    else if args_count > 2 {
        println!("Too many arguements!");
        return;
    }
    else {
        println!("Insufficient arguements");
        return;
    }
}