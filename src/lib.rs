use std::{
    env::args,
    fs::{read_dir, rename},
    path::PathBuf,
};

pub fn parse_path_args() -> Result<PathBuf, &'static str> {
    let arg = match args().skip(1).next() {
        Some(arg) => arg,
        None => return Err("No argument provided"),
    };

    let path = PathBuf::from(arg);

    if path.is_dir() {
        Ok(path)
    } else {
        return Err("Invalid path provided");
    }
}

#[derive(PartialEq, Debug)]
pub enum FileType {
    Image,
    Video,
    Other,
}
// TODO handle out of bounds after split!!!
impl FileType {
    pub fn file_type(path: &PathBuf) -> Self {
        let extension: Vec<&str> = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split(".")
            .collect();

        // [0] is filename [1] is file extention (most cases)
        let extension = extension[1].to_lowercase();

        match &extension[..] {
            "jpg" | "jpeg" | "png" | "webp" | "gif" => return Self::Image,
            "mp4" | "mov" | "webm" => return Self::Video,
            _ => return Self::Other,
        };
    }

    pub fn base_dir(self) -> Option<PathBuf> {
        let mut base_dir = PathBuf::new();

        match self {
            Self::Image => base_dir.push("TestPics"),
            Self::Video => base_dir.push("TestVids"),
            Self::Other => return None,
        };

        Some(base_dir)
    }
}
pub struct PathArgs {}

pub fn sort_into(file_name: &PathBuf) -> Option<PathBuf> {
    match FileType::file_type(file_name) {
        FileType::Image => {
            let mut to = PathBuf::from(home_path());
            let base_dir = match FileType::base_dir(FileType::Image) {
                Some(base) => base,
                None => return None,
            };
            to.push(base_dir);
            Some(to)
        }
        FileType::Video => {
            let mut to = PathBuf::from(home_path());
            let base_dir = match FileType::base_dir(FileType::Video) {
                Some(base) => base,
                None => return None,
            };
            to.push(base_dir);
            Some(to)
        }
        FileType::Other => None,
    }
}

pub fn home_path() -> PathBuf {
    let home = match home::home_dir() {
        Some(path) => path,
        None => panic!("Cannot get home dir"),
    };

    home
}

pub fn process_files(path: PathBuf) -> Result<(), &'static str> {
    let entries = match read_dir(path) {
        Ok(items) => items,
        Err(_) => return Err("{Problem reading dir"),
    };

    for item in entries {
        let from = match item {
            Ok(item) if item.path().is_file() => item.path(),
            Err(err) => panic!("{err}"),
            Ok(_) => {
                println!("Not a file. Skipping...");
                continue;
            }
        };

        let mut full_path = from.clone();
        let ending = from.file_name().unwrap();

        let mut to = match sort_into(&mut full_path) {
            Some(base_path) => base_path,
            None => {
                println!("Other file, skipping...");
                continue;
            }
        };

        to.push(PathBuf::from(ending));

        match rename(&from, &to) {
            Ok(_) => {
                println!("Success! {:?} moved to {:?}", from, to);
            }
            Err(err) => panic!("{}", err),
        };
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_path_test() {
        assert_eq!(home_path(), PathBuf::from(dotenvy::var("HOME_TEST").unwrap()));
    }

    #[test]
    fn base_dir_test() {
        assert_eq!(
            FileType::base_dir(FileType::Image).unwrap(),
            PathBuf::from("TestPics")
        );
        assert_eq!(
            FileType::base_dir(FileType::Video).unwrap(),
            PathBuf::from("TestVids")
        );

        let file_type = match FileType::base_dir(FileType::Other) {
            Some(_) => panic!("Should not be some"),
            None => (),
        };

        assert_eq!(file_type, ());
    }

    #[test]
    fn file_type_test() {
        // Image
        let file_path = PathBuf::from("/home/somefolder/image.jpg");
        assert_eq!(FileType::file_type(&file_path), FileType::Image);

        let file_path = PathBuf::from("/home/someotherfolder/anotherfolder/vid.gif");
        assert_eq!(FileType::file_type(&file_path), FileType::Image);

        //Video
        let file_path = PathBuf::from("/home/coolvid.mov");
        assert_eq!(FileType::file_type(&file_path), FileType::Video);

        let file_path = PathBuf::from("video.webM");
        assert_eq!(FileType::file_type(&file_path), FileType::Video);

        // Other
        let file_path = PathBuf::from("/idk/idk2/notes.txt");
        assert_eq!(FileType::file_type(&file_path), FileType::Other);

        let file_path = PathBuf::from("/home/somefolder/project/.gitignore");
        assert_eq!(FileType::file_type(&file_path), FileType::Other);
    }

    #[test]
    #[should_panic]
    fn file_type_test_panic() {
        let file_path = PathBuf::from("/somefile/..");
        FileType::file_type(&file_path);
    }

    #[test]
    fn sort_into_test() {
        // Image
        let res = PathBuf::from(dotenvy::var("TEST_IMGS_FULL_PATH").unwrap());
        let path = PathBuf::from("/home/somefolder/hello.jpg");

        assert_eq!(res, sort_into(&path).unwrap());

        // Video
        let res = PathBuf::from(dotenvy::var("TEST_VIDS_FULL_PATH").unwrap());
        let path = PathBuf::from("/somefolder/hello.mp4");

        assert_eq!(res, sort_into(&path).unwrap());

        // Other
        let path = PathBuf::from("/somefolder/files.zip");

        assert_eq!(None, sort_into(&path));
    }

    #[test]
    #[should_panic]
    fn sort_into_test_panic() {
        let path = PathBuf::from("/home/somefolder/readme.md");
        let _base = sort_into(&path).unwrap();
    }
}
