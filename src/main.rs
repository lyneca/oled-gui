use std::env;
use rppal::i2c::I2c;
use ssd1306::{Builder, mode::GraphicsMode};
use piscreen::{View, ButtonSet};
use piscreen::views::{MenuView, TextView, FileView, TextInputView, FuncView};
use piscreen::{menu_view, text_view, file_view};
use reqwest::blocking::Client;
use std::collections::HashMap;

const URL: &str = "https://slack.com/api/users.profile.set";

fn send_status(icon: &str, text: &str) {
    let client = Client::new();
    let mut payload = HashMap::new();
    let mut profile = HashMap::new();
    profile.insert("status_text", text);
    profile.insert("status_emoji", icon);
    profile.insert("status_expiration", "0");
    payload.insert("profile", profile);
    if let Ok(token) = env::var("SLACK_TOKEN") {
        let res = match client.post(URL)
            .json(&payload)
            .header("Authorization", format!("Bearer {}", token))
            .send() {
                Ok(res) => Some(res),
                _ => None
            };
        match res {
            Some(response) => match response.text() {
                Ok(t) => println!("{}", t),
                _ => {}
            },
            None => {}
        }
    } else {
        println!("Couldn't set status");
    }
}

macro_rules! slack_status {
    ($x:expr, $y:expr) => {
        FuncView::new(&|| {
            send_status($y, $x);
        })
    };
}

fn main() {
    let mut i2c = I2c::new().expect("Could not create I2C Device");
    i2c.set_slave_address(0x3c).expect("Could not select device");

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    disp.init().expect("Could not init device");
    let mut buttons = ButtonSet::default_pins();

    let mut root = menu_view![
        ("Files", file_view!("/home/pi")),
        ("Input", TextInputView::new()),
        // ("Wifi", WifiView::new()),
        ("Set Slack Status", menu_view![
            ("Available", slack_status!("Available", ":successful:")),
            ("Eating lunch", slack_status!("Eating lunch", ":apple:")),
            ("Sleeping", slack_status!("Sleeping", ":sleeping:")),
            ("Piano break", slack_status!("Piano break", ":musical_keyboard:")),
            ("Bugfixing", slack_status!("Bugfixing", ":bug:")),
            ("Helping with a HOT", slack_status!("Helping with a HOT", ":hot:")),
            ("Oncall", slack_status!("Oncall", ":telephone_receiver:")),
            ("Walking dog", slack_status!("Walking dog", ":doggoblob:"))
        ]),
        ("Text Tests", menu_view![
            ("Captain Vor's Speech", text_view!(
                "Look at them, they come to this place when they know they are not pure.
                Tenno use the keys, but they are mere trespassers. Only I, Vor, know the
                true power of the Void. I was cut in half, destroyed, but through it's
                Janus Key, the Void called to me. It brought me here and here I was
                reborn. We cannot blame these creatures, they are being led by a false
                prophet, an impostor who knows not the secrets of the Void. Behold the
                Tenno, come to scavenge and desecrate this sacred realm. My brothers,
                did I not tell of this day? Did I not prophesize this moment? Now, I
                will stop them. Now I am changed, reborn through the energy of the
                Janus Key.  Forever bound to the Void. Let it be known, if the Tenno
                want true salvation, they will lay down their arms, and wait for the
                baptism of my Janus key. It is time. I will teach these trespassers the
                redemptive power of my Janus key. They will learn it's simple truth. The
                Tenno are lost, and they will resist. But I, Vor, will cleanse this
                place of their impurity."
            )),
            ("Tragedy of Darth Plagueis the Wise", text_view!(
                "Did you ever hear the tragedy of Darth Plagueis The Wise? I thought
                not. It's not a story the Jedi would tell you. It's a Sith legend.
                Darth Plagueis was a Dark Lord of the Sith, so powerful and so wise
                he could use the Force to influence the midichlorians to create
                life... He had such a knowledge of the dark side that he could even
                keep the ones he cared about from dying. The dark side of the Force
                is a pathway to many abilities some consider to be unnatural. He
                became so powerful… the only thing he was afraid of was losing his
                power, which eventually, of course, he did. Unfortunately, he taught
                his apprentice everything he knew, then his apprentice killed him in
                his sleep. Ironic. He could save others from death, but not himself."
            ))
        ])
    ];


    loop {
        disp.clear();
        buttons.poll_all();
        root.render(&mut disp);
        root.handle_buttons(&mut buttons);
        buttons.flush();
        disp.flush().unwrap();
    }
}
