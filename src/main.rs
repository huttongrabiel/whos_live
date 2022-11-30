use std::env;
use tokio;

struct TwitchRequest {
    url: String,
}

impl TwitchRequest {
    fn new(username: &String) -> Self {
        Self {
            url: format!("https://www.twitch.tv/{}", username),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    let mut args = env::args().into_iter();
    args.next();

    let username = match args.next() {
        // Twitch API uses all lowercase names for streamers.
        Some(username) => username.to_lowercase(),
        None => return Err("Please provide a twitch stream name!"),
    };

    let request = TwitchRequest::new(&username);
    let response = make_request(&request).await;

    if is_live(response) {
        display_live_user(username, request.url)
    } else {
        println!("{} is not live. :(", username)
    }

    Ok(())
}

/// Query the url of a twitch stream at "twitch.tv/{username}".
///
/// Returns html contents of page when can be searched to determine
/// whether a streamer is live or not.
///
/// # Panics
///
/// Potential for await to return an error. Because of Twitch's design, this
/// should never panic, but there is always a possibility of Twitch changing
/// things on their end.
async fn make_request(request_content: &TwitchRequest) -> String {
    let response = reqwest::get(&request_content.url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    response
}

/// Looks through the response returned by make_request for "isLiveBroadcast".
///
/// isLiveBroadcast only appears in the response when the streamer is live.
fn is_live(response: String) -> bool {
    // No Twitch API in this town! Only live channles have isLiveBroadcast in
    // the inspect source. 100000 IQ
    if response.contains("isLiveBroadcast") {
        return true;
    }
    false
}

/// Prints a box with "<username> is live!" and the link to the stream in it.
///
/// # Panics
///
/// Slicing the string could panic if the character is not ascii.
fn display_live_user(username: String, url: String) {
    // + 4 accounts for the border of the square and the space on each side.
    let x = url.len() + 4;
    let y = 6;

    let live_message = format!("{} is live!", username);
    let live_message_start_index = (x - live_message.len()) / 2;

    for i in 0..y {
        for j in 0..x {
            if i == 0 || i == y - 1 || j == 0 || j == x - 1 {
                print!("*");
            } else if i == 2
                && (j > live_message_start_index
                    && j < live_message.len() + live_message_start_index)
            {
                let index = j - live_message_start_index - 1;
                print!("{}", live_message.get(index..index + 1).unwrap());
            } else if i == 3 && (j > 1 && j < x - 2) {
                let index = j - 2;
                print!("{}", url.get(index..index + 1).unwrap());
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
