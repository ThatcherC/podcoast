use rss::extension::itunes::ITunesChannelExtension;
use rss::ChannelBuilder;

fn main() {
    let channel = ChannelBuilder::default()
        .title("Channel Title")
        .link("http://example.com")
        .description("An RSS feed.")
        .build();
    //    .unwrap();

    let mut channelext = ITunesChannelExtension::default();
    channelext.set_author("John Doe".to_string());
    channelext.set_summary("Weather pod!".to_string());

    let ituneschannel = ChannelBuilder::default()
        .itunes_ext(channelext)
        .title("Channel Title")
        .link("http://example.com")
        .description("An RSS feed.")
        .build();

    let writer = ::std::io::stdout();
    //    channel.write_to(writer).unwrap(); // // write to the channel to a writer

    // pretty writer from https://docs.rs/rss/latest/rss/struct.Channel.html#example-2
    channel.pretty_write_to(writer, b' ', 2).unwrap(); // // write to the channel to a writer
    let string = channel.to_string(); // convert the channel to a string

    println!("");
    println!("");
    println!("{}", string);
    println!("");
    println!("");
    ituneschannel
        .pretty_write_to(::std::io::stdout(), b' ', 2)
        .unwrap();
    println!("");
    println!("");
}
