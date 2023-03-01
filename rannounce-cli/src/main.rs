use clap::Parser;
use new_rawr::{auth, client, options::LinkPost, options::SelfPost};
use rpassword::prompt_password;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    title: String,

    #[arg(long)]
    link: Option<String>,

    #[arg(long)]
    text: Option<String>,

    username: String,

    client_id: String,

    #[arg(long)]
    nsfw: bool,

    subreddits: Vec<String>,
}

fn main() {
    let args = Args::parse();
    if !((args.link == None) ^ (args.text == None)) {
        println!("Please enter exactly one of --link or --text");
        std::process::exit(0)
    }

    let client = client::RedditClient::new(
        &format!("{}:rannounce:v0.1.0 (by /u/__Wolfie)", std::env::consts::OS),
        auth::PasswordAuthenticator::new(
            &args.client_id,
            &prompt_password(
                "please enter your secret code (see docs if you don't know what this means)",
            )
            .expect("User did not input a secret"),
            &args.username,
            &prompt_password("please enter your Reddit password")
                .expect("User did not enter a password"),
        ),
    );
    if let Some(link) = args.link {
        for subreddit in args.subreddits.iter() {
            let post = LinkPost::new(&args.title, &link);
            let subreddit = client.subreddit(&subreddit);
            let body = format!(
                "api_type=json&extension=json&kind=link&resubmit={}&sendreplies=true&\
                    sr={}&title={}&url={}&nsfw={}",
                post.resubmit,
                subreddit.name,
                client.url_escape(post.title.to_owned()),
                client.url_escape(post.link.to_owned()),
                args.nsfw.to_string()
            );
            client
                .post_success("/api/submit", &body, false)
                .expect("Was not able to submit link post");
        }
    }

    if let Some(text) = args.text {
        for subreddit in args.subreddits.iter() {
            let post = SelfPost::new(&args.title, &text);
            let subreddit = client.subreddit(&subreddit);
            let body = format!(
                "api_type=json&extension=json&kind=self&sendreplies=true&\
                    sr={}&title={}&url={}&nsfw={}",
                subreddit.name,
                client.url_escape(post.title.to_owned()),
                client.url_escape(post.text.to_owned()),
                args.nsfw.to_string()
            );
            client
                .post_success("/api/submit", &body, false)
                .expect("Was not able to submit text post");
        }
    }
}
