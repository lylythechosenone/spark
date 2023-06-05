//   ______                                      _       _   
// |  ____|                                    (_)     | |  
// | |__ __ _ _ __   ___ _   _       _ __  _ __ _ _ __ | |_ 
// |  __/ _` | '_ \ / __| | | |     | '_ \| '__| | '_ \| __|
// | | | (_| | | | | (__| |_| |     | |_) | |  | | | | | |_ 
// |_|  \__,_|_| |_|\___|\__, |     | .__/|_|  |_|_| |_|\__|
//                        __/ |_____| |                     
//                        |___/______|_|                     
// 
// ==========================================================
// 
// 
// Custom inbuilt TUI libary for the spark Package Manager 

use termion::color::{AnsiValue, Fg, Reset};

#[allow(dead_code)]
pub fn colorize(color: u8, message: &str) -> String {
    format!("{}{}{}", Fg(AnsiValue(color)), message, Fg(Reset))
}

