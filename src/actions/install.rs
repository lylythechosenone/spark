use crate::actions::fancyprint;
/// format for packages:
/// /directory/to/package@channel:version
///
/// example:
///     example/helloworld@stable:1.0
///

// TODO: parse package string and install.
pub fn handle(package: &mut String) {
    let mut pkg_path = String::new();
    let mut pkg_channel = String::new();
    let mut pkg_version = String::new();

    if let Some((path, channel_version)) = package.split_once('@') {
        pkg_path = path.to_string();
        if let Some((channel, version)) = channel_version.split_once(':') {
            pkg_channel = channel.to_string();
            pkg_version = version.to_string();
        }
    }

    if pkg_path.is_empty() || pkg_channel.is_empty() || pkg_version.is_empty() {
        pkg_path = package.split('@').next().unwrap().to_string();
        pkg_channel = "stable".to_string();
        pkg_version = "latest".to_string();

        println!(
            "{}{}{}",
            fancyprint::colorize(196, "> Error: invalid package format, defaulting to: "),
            fancyprint::colorize(93, &pkg_path.as_str()),
            fancyprint::colorize(93, "@stable:latest"),
        );
    }


}
