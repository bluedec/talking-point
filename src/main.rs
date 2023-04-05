
use cursive::Cursive;
use cursive::views::Dialog;
use something_interesting::something_interesting;
use std::env;

fn main() {
    let opened_count = env::var("OPENED_COUNT").unwrap_or_default();
    let mut counter = opened_count.parse::<u32>().unwrap_or_default();
    counter+= 1;
    env::set_var("OPENED_COUNT", counter.to_string());
    let mut siv = cursive::default();

    siv.add_global_callback('q', Cursive::quit);
    let mut hi = "Hello user";
    if counter > 0 && counter < 10 {
        hi = "Hi again pancake"; 
    } else if counter > 10 && counter < 25 {
        hi = "Hi youu :3";
    }
    siv.add_layer(
        Dialog::text(hi)
        .title("Welcome")
        .button("Hope you have something interesting.", |s| something_interesting(s))
        .button("Don't talk to me in that tone!", |s| that_tone())
        .button("Hello master.", |s| good_boy())
    );

    siv.run();

}


fn that_tone() {

}

fn good_boy() {

}

