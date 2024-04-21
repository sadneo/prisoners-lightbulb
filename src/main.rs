use rand::{self, RngCore};
use std::{thread, time};

fn main() {
    let mut prisoners = [false; 99];
    let mut count = 0;
    let mut bulb_on = false;
    let mut iteration = 0;
    let five_seconds = time::Duration::from_secs(5);
    let five_millis = time::Duration::from_millis(5);

    let mut rng = rand::thread_rng();

    println!("Enjoy the visual!\n(Prisoners are rendered after they make their choice)\nIteration,Current Count,Bulb State, Prisoners\n");
    thread::sleep(five_seconds);
    
    render(&prisoners, &iteration, &count, &bulb_on);
    while count < 99 {
        let number = (rng.next_u64() % 100) as usize;
        if number == 99 {
            if bulb_on {
                count += 1;
                bulb_on = false;
            }
        } else {
            let prisoner_went = prisoners[number];
            if !prisoner_went && !bulb_on {
                prisoners[number] = true;
                bulb_on = true;
            }
        }
        iteration += 1;
        render(&prisoners, &iteration, &count, &bulb_on);
        thread::sleep(five_millis);
    }
}

fn render(prisoners: &[bool], iteration: &u32, count: &u8, bulb_on: &bool) {
    let bulb = if *bulb_on {" on"} else {"off"};
    print!("{:>9} {:>2} {}", iteration, count, bulb);
    for prisoner in prisoners {
        if *prisoner {
            print!("\x1b[2;90;101mX");
        } else {
            print!("\x1b[2;90;102mX");
        }
    }
    println!("\x1b[0m");
}
