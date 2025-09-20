#![warn(clippy::all, clippy::pedantic)]



/* #####################
  ##    CONSTANTS    ##
  #####################*/



const DEFAULT_PATH_SUFFIX: &str = "";
const DEFAULT_NAME_PREFIX: &str = "shotrs-";
const DEFAULT_NAME_SUFFIX: &str = ".png";



/*###################
  ##    IMPORTS    ##
  ###################*/



//--Parser for flags--
use clap::Parser;
//--Std for filesystem interaction and getting home directory path--
use std::env;
use std::fs;
//--XCap for capturing screen--
use xcap::Monitor;
//--Chrono for setting --
use chrono::Utc;



/*######################
  ##    STRUCTURES    ##
  ######################*/



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///Path where to store screenshots
    #[arg(short, long, default_value_t = format!("{}{}", env::home_dir().unwrap().to_str().unwrap().to_string(), DEFAULT_PATH_SUFFIX))]
    path: String,

    ///Name of the file
    #[arg(short, long, default_value_t = format!("{}{}", DEFAULT_NAME_PREFIX.to_string(), Utc::now().format("%H:%M:%S-%d.%m.%Y")))]
    name: String
}



/*################
  ##    MAIN    ##
  ################*/



fn main() {
    //--Parsing flags--
    let args = Args::parse();

    //--Creating directory for screenshots if not exist--
    match fs::create_dir_all(&args.path) {
        Ok(_) => {
            println!("Folder \"{}\" already exist or was succesfully created", &args.path);
            capture_screen(args.path, args.name);
        },
        Err(error) => eprintln!("Error creating new folder: {error}"),
    };

    
}



/*#####################
  ##    FUNCTIONS    ##
  #####################*/



//----Capture screen----
fn capture_screen(path: String, name: String) {
    //--Getting monitors--
    let monitors = Monitor::all().expect("Error getting monitors!");

    //--Loop for taking screenshot of every monitor--
    for monitor in monitors {
        //--Capture image--
        let image = monitor.capture_image().expect("Error capturing image!");

        //--Create a name for screenshot--
        let screenshot_name = format!("{}/{}-monitor:{}{}", path, name, monitor.name().unwrap(), DEFAULT_NAME_SUFFIX);

        //--Save screenshot--
        match image.save(&screenshot_name) {
            Ok(_) => println!("Screenshot was taken and saved at \"{screenshot_name}\""),
            Err(error) => eprintln!("Error taking and saving screenshot at \"{screenshot_name}\": {error}"),
        };
    }
}

// TODO: make function for capturing frames