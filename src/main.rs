use rss::extension::itunes::{NAMESPACE, ITunesChannelExtensionBuilder};
use rss::{ChannelBuilder, EnclosureBuilder, ItemBuilder};
use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use url::{Url,ParseError};

#[macro_use]
extern crate clap;
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const RSSPATH: &'static str = "rss/";
const IMAGEPATH: &'static str = "img/";
const AUDIOPATH: &'static str = "audio/";

fn create_output_structure(path: &PathBuf) {
    fs::create_dir_all(path.join(RSSPATH)).unwrap();
    fs::create_dir_all(path.join(IMAGEPATH)).unwrap();
    fs::create_dir_all(path.join(AUDIOPATH)).unwrap();
}

fn channelfromdir(config: &serde_yaml::Value) -> rss::ChannelBuilder {
    let linkurl = config["link"]
        .as_str()
        .map(|s| s.to_string())
        .expect("Could not find key link in something.yaml");
    println!("Link: {}", linkurl);

    let description = config["description"]
        .as_str()
        .map(|s| s.to_string())
        .expect("Could not find key description in something.yaml");
    println!("Desc: {}", description);

    let title = config["title"]
        .as_str()
        .map(|s| s.to_string())
        .expect("Could not find key title in something.yaml");
    println!("Title: {}", title);
    
    let baseurl = config["baseurl"]
        .as_str()
        .expect("Could not find key baseurl in something.yaml");
    println!("Base URL: {}", baseurl);
    
    let imagename = config["imagename"]
        .as_str()
        .expect("Could not find key imagename in something.yaml");
    println!("Image: {}", imagename);
    
    let imageurl = Url::parse(baseurl).unwrap().join(IMAGEPATH).unwrap().join(imagename).unwrap().as_str().to_string();
    
    println!("Image URL: {}", imageurl.clone());
    
    let mut channelext = ITunesChannelExtensionBuilder::default()
        .image(imageurl)
        .author("John Doe".to_string())
        .summary(description)
        .build();

    let ituneschannel = ChannelBuilder::default()
        .namespace(("itunes".to_string(), NAMESPACE.to_string())) // NOTE: should this be required? might be a bug in rss crate
        .itunes_ext(channelext)
        .title(title)
        .link(linkurl)
        .clone();
    if let Some(image) = ituneschannel.clone().build().itunes_ext().expect("no itunes").image().as_ref() {
        println!("It's a some!");
    }else{
        println!("It's *not* a some!");
    }
        
    
    println!("Channel image: {}", ituneschannel.clone().build().itunes_ext().expect("no itunes").image().expect("bad iamge?"));
    
    return ituneschannel;
}

fn enclosurefromfile(path: &PathBuf) -> Option<rss::Enclosure> {
    //TODO! build a (dummy?) enclosure object from a filename
    // use rodio to get duration
    // match fileformat with infer to get mime type
    Some(EnclosureBuilder::default().build())
}

fn episodefromdir(path: &PathBuf) -> Option<rss::Item> {
    // build a dummy episode that always succeeds using the filename
    Some(
        ItemBuilder::default()
            .title(
                path.file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .ok()?,
            )
            .description(
                path.canonicalize()
                    .ok()?
                    .into_os_string()
                    .into_string()
                    .ok()?,
            )
            .enclosure(enclosurefromfile(path)?)
            .build(),
    )
}

fn main() -> io::Result<()> {
    let matches = clap_app!(myapp =>
        (version: VERSION)
        (author: "Thatcher Chamberlin <j.thatcher.c@gmail.com>")
        (about: "Turn a directory of audio files into a podcast feed")
        (@arg OUTPUTDIR: -o --output +required +takes_value "Sets output directory")
        (@arg INPUTDIR:  -i --input  +required +takes_value "Sets input directory")
        //(@arg debug: -d ... "Sets the level of debugging information")
    )
    .get_matches();
    
    

    let outputdirectory = PathBuf::from(matches.value_of("OUTPUTDIR").unwrap());
    create_output_structure(&outputdirectory);

    let inputdirectory = matches.value_of("INPUTDIR").unwrap();
    
    let configpath = PathBuf::from(inputdirectory).join("channel.yaml");
    let f = std::fs::File::open(configpath).expect("Unable to open config file!");
    let config: serde_yaml::Value = serde_yaml::from_reader(f).expect("Unable to deserialize!");
    
    //create channel from config
    let mut ituneschannel = channelfromdir(&config);
    println!("DOne");

    // iterate over directories in input directory
    let mut inputentries = fs::read_dir(inputdirectory)?
        // map only applies the function if the element of the iterator is an Ok!
        // since the items `res` are not lists, we can just use and_then to avoid map
        // & the connotation of iterating
        .map(|res| res.and_then(|e| Ok(e.path())))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let inputdirectories = inputentries
        .iter()
        .filter(|e| e.is_dir())
        .collect::<Vec<_>>();

    println!("Entries    : {:?}", inputentries);
    println!("Directories: {:?}", inputdirectories);

    let episodes = inputdirectories
        .iter()
        .filter_map(|path| episodefromdir(&path))
        .clone()
        .collect::<Vec<_>>();

    ituneschannel.items(episodes);

    let path = outputdirectory.join(RSSPATH).join("podcast.rss");
    println!("Writing to '{}'", path.display());
    let writer = File::create(&path).unwrap();

    //    channel.write_to(writer).unwrap(); // // write to the channel to a writer

    println!("");
    println!("");
    ituneschannel
        .build()
        .pretty_write_to(writer, b' ', 2)
        .unwrap();
    println!("");
    println!("");

    Ok(())
}
