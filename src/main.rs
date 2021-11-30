	use rss::ChannelBuilder;

fn main() {

let channel = ChannelBuilder::default()
    .title("Channel Title")
    .link("http://example.com")
    .description("An RSS feed.")
    .build();
//    .unwrap();
    

    let writer = ::std::io::stdout();
//    channel.write_to(writer).unwrap(); // // write to the channel to a writer

    // pretty writer from https://docs.rs/rss/latest/rss/struct.Channel.html#example-2
    channel.pretty_write_to(writer, b' ', 2).unwrap(); // // write to the channel to a writer
    let string = channel.to_string(); // convert the channel to a string

    println!("");
    println!("");
    println!("{}", string);
}
