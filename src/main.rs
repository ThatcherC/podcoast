use rss::extension::itunes::{ITunesChannelExtensionBuilder, NAMESPACE};
use rss::{ChannelBuilder, EnclosureBuilder, ItemBuilder};
use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use url::{ParseError, Url};

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

    let imageurl = Url::parse(baseurl)
        .unwrap()
        .join(IMAGEPATH)
        .unwrap()
        .join(imagename)
        .unwrap()
        .as_str()
        .to_string();

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

    println!(
        "Channel image: {}",
        ituneschannel
            .clone()
            .build()
            .itunes_ext()
            .expect("no itunes")
            .image()
            .expect("bad iamge?")
    );

    return ituneschannel;
}

fn enclosurefromfile(
    config: &serde_yaml::Value,
    outputdirectory: &PathBuf,
    path: &PathBuf,
) -> Result<rss::Enclosure, Box<dyn std::error::Error>> {
    //TODO! build a (dummy?) enclosure object from a filename
    // use rodio to get duration
    // match fileformat with infer to get mime type

    let filetype = infer::get_from_path(path)
        .expect("file read successfully")
        .expect("file type is known");

    let metadata = fs::metadata(path)?;

    let baseurl = config["baseurl"]
        .as_str()
        .expect("Could not find key baseurl in something.yaml");

    // path of audio file in the input structure
    let filename = Path::new(path).file_name().unwrap().to_str().unwrap();

    let episoderelativepath = PathBuf::from(AUDIOPATH)
        .join(filename);
    

    // path of episode audio file in the output structure
    let episodeoutputpath = PathBuf::from(outputdirectory)
        .join(&episoderelativepath)
        .to_str()
        .unwrap()
        .to_string();

    // URL of the audio file in the deployed podcast structure
    let episodeurl = Url::parse(baseurl)
        .unwrap()
        .join(&episoderelativepath.to_str()
            .unwrap()
            .to_string())
        .unwrap()
        .as_str()
        .to_string();

    //println!("Episode input path:  {:?}", path.clone());
    //println!("Episode output path: {}", episodepath.clone());

    println!("Copying {:?} to {}", path.clone(), episodeoutputpath.clone());
    fs::copy(path.clone(), episodeoutputpath.clone());

    println!("Episode URL: {}", episodeurl.clone());

    println!("Build enclosure! {} bytes", metadata.len().to_string());

    // TODO! copy audio file to output tree
    // Ok(enclosure.build())
    Ok(EnclosureBuilder::default()
        .mime_type(filetype.mime_type())
        .length(metadata.len().to_string())
        .url(episodeurl)
        .build())
}

fn isokayaudio(mimetype: &str) -> bool {
    println!("Check whether mimetype {} matches audio/wav", mimetype);

    mimetype == "audio/wav" || mimetype == "audio/x-wav"
}

fn episodefromdir(
    config: &serde_yaml::Value,
    outputdirectory: &PathBuf,
    path: &PathBuf,
) -> Result<rss::Item, Box<dyn std::error::Error>> {
    // build a dummy episode that always succeeds using the filename
    let pathstr = path
        .canonicalize()?
        .into_os_string()
        .into_string()
        .map_err(|e| "couldn't stringify")?;

    // Produce a title for the episode
    // TODO: remove dummy title which is just the filename
    let title = path
        .file_name()
        .ok_or("coudn't get path name!")?
        .to_os_string()
        .into_string()
        .map_err(|e| "couldn't stringify")?;

    println!("");
    println!("{:?}", path);

    let files = fs::read_dir(path)?
        // map only applies the function if the element of the iterator is an Ok!
        // since the items `res` are not lists, we can just use and_then to avoid map
        // & the connotation of iterating
        .map(|res| res.and_then(|e| Ok(e.path())))
        .collect::<Result<Vec<_>, io::Error>>()?;

    println!("List of files: {:?}", files);

    let audiofiles = files
        .iter()
        .filter(|e| {
            isokayaudio(
                infer::get_from_path(e)
                    .expect("file read successfully")
                    .expect("file type is known")
                    .mime_type(),
            )
        })
        .collect::<Vec<_>>();

    println!("List of audio files: {:?}", audiofiles);

    // TODO! check if more than one audio file exists. this assumes there's one
    // and uses the first
    let audiofile = audiofiles[0];
    //    .iter()
    //    .next()
    //    .ok_or("No audio files available!")?;

    println!("Selected audio file: {:?}", audiofile);

    Ok(ItemBuilder::default()
        .title(title)
        .description(pathstr)
        .enclosure(enclosurefromfile(config, outputdirectory, audiofile)?)
        .build())
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
    let baseurl = config["baseurl"]
        .as_str()
        .expect("Could not find key baseurl in something.yaml");

    //create channel from config
    let mut ituneschannel = channelfromdir(&config);

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
        .filter_map(|path| episodefromdir(&config, &outputdirectory, &path).ok())
        .clone()
        .collect::<Vec<_>>();

    ituneschannel.items(episodes);

    let path = outputdirectory.join(RSSPATH).join("podcast.rss");
    let writer = File::create(&path).unwrap();

    //    channel.write_to(writer).unwrap(); // // write to the channel to a writer

    println!("");
    println!("");
    ituneschannel
        .build()
        .pretty_write_to(writer, b' ', 2)
        .unwrap();
    println!("");

    println!("Wrote to '{}'", path.display());
    println!(
        "Copy {}/* to {} to publish!",
        outputdirectory
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .expect("couldn't format"),
        baseurl
    );
    println!(
        "And subscribe to {}{}podcast.rss to subscribe!",
        baseurl, RSSPATH
    );

    Ok(())
}
