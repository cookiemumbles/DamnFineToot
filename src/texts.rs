use rand::seq::SliceRandom;

const STATEMENTS: &'static [&str] = &[
    "Damn fine indeed!",
    "Whoa!",
    "Whoa nelly!",
    "Wow!",
    "Unbelievable!",
    "Can you believe it?",
    "Isn't it lovely?",
    "Oh happy day!",
    "I couldn't be happier.",
    "I think it is the right choice.",
    "Great selection!",
    "Great choice!",
    "A fine pick!",
    "It's a great day!",
    "That toot tho.",
    "What that toot do?",
    "Amazing stuff!",
    "Great stuff!",
    "Such good words.",
    "How do you like that?",
    "Congratulations!",
    "Boy howdy!",
    "I cannot disagree.",
    "Well what do you know.",
    "Nice!",
    "Noice!",
    "Daaaaang!",
    "It was a long time coming!",
    "It's a good one folks.",
    "I mean, it's so true.",
    "Hey now!",
    "It is known.",
    "Best toot since sliced bread.",
    "This is even better than that other one.",
    "Ain't it a peach?",
    "You betcha!",
    "You never think it will happen to you.",
    "Better than Shakespeare.",
    "When it's good it's good.",
    "Ho. Lee. Shit.",
    "WHOAH!!!",
    "Shut the front door!",
    "I'm super excited!",
    "Stop the presses!",
    "Ding ding ding!",
    "We have a winner!",
    "Oooo it's one of my favorites.",
    "Egad!",
    "What a toot!",
    "This one was overdue.",
    "It's so fetch!",
    "Yessirree!",
    "It's a work of art.",
    "They like you. They really like you.",
    "Inconceivable!",
    "Without doubt excellent.",
    "Simply the best.",
    "Almost as good as Murder She Wrote.",
    "Huzzah!",
    "What's not to like?",
    "I laughed, I cried.",
    "Five stars, would read again.",
    "Booyah!",
    "One toot to rule them all.",
    "Let's celebrate!",
    "Stunning bit of prose.",
    "Significantly better than Ezra.",
    "Fancy that!",
    "Good toot or best ever?",
    "Dios mio!",
    "It is without peer.",
    "EEEEEEEE!",
    "Ohhhhh yeah!",
    "Yo Adrian!",
    "Like a fine wine.",
    "Valar dohaeris!",
    "A masterpiece.",
    "Makes me want to dance.",
    "Outstanding!",
    "Open the champagne!",
    "Oofda dat's a good one!",
    "Whoa Nelly, would you look at that!",
    "Feels so good!",
    "That's the stuff!",
    "Excelsior!",
    "Praise be!",
    "Hot damn!",
    "Brilliant!",
    "A work of genius.",
    "Cheers!",
    "So freaking cool.",
    "Just as I expected.",
    "Fantastico!",
    "Hip hip hurray!",
    "Well well well.",
    "The Duke's mayonnaise of tweets.",
    "Glorious!",
    "Tweetariffic!",
    "Wish I had thought of it.",
    "Clever!",
    "That's some toot!",
    "The quality we deserve.",
    "So choice!",
    "Gold star!",
    "A work of great literature, really.",
    "Truly something to behold.",
    "We could use more like this one.",
    "Like a ray of sunshine!",
    "Truly inspirational.",
    "On a scale of 1 to 10: 11.",
    "I am so proud.",
    "Such a refreshing perspective.",
    "Simply perfect.",
    "This is the way.",
    "I like good tweets and I cannot lie.",
    "I gotta read that one again!",
    "Never mind DFT, Pulitzer for this one!",
    "A special unicorn of a toot.",
    "A real gem.",
    "Clearly the best one today.",
    "Where has this toot been all my life?",
    "Swipe right!",
    "I tip my chapeau!",
    "Stand up and holla!",
    "Standing ovation!",
    "Take a victory lap!",
    "Goodness gracious!",
    "I don't say this to just anyone.",
    "Pretty, pretty, pretty, pretty good.",
    "Dude.",
    "Better than Improv Night!",
];

pub fn format_dft_toot(receiver: &str, sender: &str, toot_url: &str) -> String {
    let handle_texts = [
        format!(
            "{} 's pick for toot of the day is by {} .",
            sender, receiver
        ),
        format!(
            "{} 's toot was selected by {} as the toot of the day.",
            receiver, sender
        ),
        format!("{} named {} 's toot the best of the day.", sender, receiver),
        format!("{} picked you, {} .", sender, receiver),
        format!("{} selected {} 's toot.", sender, receiver),
        format!("{} was trophied by {} .", receiver, sender),
    ];
    let selected_statements: Vec<&str> = STATEMENTS
        .choose_multiple(&mut rand::thread_rng(), 3)
        .cloned()
        .collect();
    return format!(
        "{} {} {} {}\n{}",
        selected_statements[0],
        selected_statements[1],
        handle_texts.choose(&mut rand::thread_rng()).unwrap(),
        selected_statements[2],
        toot_url
    );
}
