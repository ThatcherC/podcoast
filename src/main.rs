use rss::extension::itunes::ITunesChannelExtension;
use rss::ChannelBuilder;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

#[macro_use]
extern crate clap;
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn channelfromdir(path: &PathBuf) -> rss::ChannelBuilder {
    let f = std::fs::File::open(path).expect("Unable to open file!");
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
    let matches = clap_app!(myapp =>
        (version: VERSION)
        (author: "Thatcher Chamberlin <j.thatcher.c@gmail.com>")
        (about: "Turn a directory of audio files into a podcast feed")
        (@arg OUTPUT: -o --output +takes_value "Sets output directory")
        (@arg INPUTDIR:  -i --input  +required +takes_value "Sets input directory")
        //(@arg debug: -d ... "Sets the level of debugging information")
    )
    .get_matches();

    let inputpath = PathBuf::from(matches.value_of("INPUTDIR").unwrap()).join("channel.yaml");

    let ituneschannel = channelfromdir(&inputpath).build();

    let writer = match matches.value_of("OUTPUT") {
        Some(filename) => {
            let path = Path::new(filename);
            println!("Writing to '{}'", path.display());
            Box::new(File::create(&path).unwrap()) as Box<dyn Write>
        }
        None => Box::new(::std::io::stdout()) as Box<dyn Write>,
    };
    //    channel.write_to(writer).unwrap(); // // write to the channel to a writer

    println!("");
    println!("");
    ituneschannel.pretty_write_to(writer, b' ', 2).unwrap();
    println!("");
    println!("");
}
