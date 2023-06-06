/// Stores information about a package.
/// stores info such as:
/// - name
/// - description
/// - versions
/// - Channels
/// - dependencies

pub struct Package {
    // UID: String,
    pub name: String,
    pub description: String,
    pub channels: Vec<Channel>,
    pub dependencies: Vec<Package>,
}

pub enum Channel {
    Alpha(Vec<Version>),
    Beta(Vec<Version>),
    Dev(Vec<Version>),
    Nightly(Vec<Version>),
    Stable(Vec<Version>),
}


impl Package {
    pub fn new(name: String, description: String, channels: Vec<Channel>, dependencies: Vec<Package>) -> Package {
        Package {
            name,
            description,
            channels,
            dependencies,
        }
    }
}

pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Version {
    pub fn strnew(str: String) -> Version {
        let mut ver = Version {
            major: 0,
            minor: 0,
            patch: 0,
        };
        let mut i = 0;
        for s in str.split(".") {
            match i {
                0 => ver.major = s.parse::<u8>().unwrap(),
                1 => ver.minor = s.parse::<u8>().unwrap(),
                2 => ver.patch = s.parse::<u8>().unwrap(),
                _ => panic!("Invalid version string"),
            }
            i += 1;
        }
        ver
    }
    pub fn new(major: u8, minor: u8, patch: u8) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn version_strnew() {
        let version_str = String::from("3.2.4");
        let version = Version::strnew(version_str);
        assert_eq!(version.major, 3);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 4);
    }
}
