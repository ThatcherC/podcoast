use rss::extension::itunes::ITunesChannelExtension;
use rss::ChannelBuilder;

fn channelfromfile() -> rss::ChannelBuilder{
    let f = std::fs::File::open("channel.yaml").expect("Unable to open file!");
    let data: serde_yaml::Value = serde_yaml::from_reader(f).expect("Unable to deserialize!");

    let linkurl = data["link"]
        .as_str()
        .map(|s| s.to_string())
        .expect("Could not find key link in something.yaml");
    println!("Link: {}", linkurl);


    let description = data["description"]
        .as_str()
        .map(|s| s.to_string())
        .expect("Could not find key description in something.yaml");
    println!("Desc: {}", description);

    let title = data["title"]
        .as_str()
        .map(|s| s.to_string())
        .expect("Could not find key title in something.yaml");
    println!("Title: {}", title);

    let mut channelext = ITunesChannelExtension::default();
    channelext.set_author("John Doe".to_string());
    channelext.set_summary(description);

    let ituneschannel = ChannelBuilder::default()
        .itunes_ext(channelext)
        .title(title)
        .link(linkurl)
        .clone();

    return ituneschannel;
}



fn main() {
    let ituneschannel = channelfromfile().build();

    let writer = ::std::io::stdout();
    //    channel.write_to(writer).unwrap(); // // write to the channel to a writer

    println!("");
    println!("");
    ituneschannel
        .pretty_write_to(writer, b' ', 2)
        .unwrap();
    println!("");
    println!("");
}
