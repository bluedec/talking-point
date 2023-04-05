
#![allow(unused_variables)]  
#![allow(dead_code)]
use cursive::Cursive;
use cursive::views::{ Button, Dialog, LinearLayout };

pub fn something_interesting(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("I actually do.")
        .title("Welcome")
        .button("What is it?", |s| wanna_play_a_game(s))
        .button("I don't want it.", |s| s.quit())
    );
}

pub fn wanna_play_a_game(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("A game. It's called 'choose eight and become the owner of a sword'")
        .button("I want to play.", |s| ok_play(s))
        .button("I don't want to play.", |s| ())
    );
}

fn ok_play(s: &mut Cursive) {
    fn choice(s: &mut Cursive) {
        s.pop_layer();
        let button = Button::new("I'm a button!", |s| s.quit());
        let button2 = Button::new("I'm also a button...", |s| s.quit());
        s.add_layer(
            LinearLayout::vertical()
            .child(button)
            .child(button2)
            
        );

    }
    s.pop_layer();
    s.add_layer(
        Dialog::text("Ok then... good luck.")
        .title("choose eight and become the owner of a sword")
        .button("Thanks...", |s| choice(s))

    );
}



pub fn quit_with_mssg(s: &mut Cursive, mssg: &str) {
    s.add_layer(
        Dialog::text(mssg).title("End").button("Ok...", |s| s.quit())
    );

}


#[cfg(test)]
mod tests {
    use super::*;

}
