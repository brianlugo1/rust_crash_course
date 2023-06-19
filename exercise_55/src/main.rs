struct SoundData {
    name: String
}

impl SoundData {
    fn new(message: String) -> Self {
        Self { name: message }
    }
}

fn get_sound(name: &str) -> Result<SoundData, String> {
    if name == "alert" {
        Ok(SoundData::new("alert".to_owned()))
    } else {
        Err("unable to find sound data".to_owned())
    }
}

fn match_sound(sound: Result<SoundData, String>) {
    match sound {
        Ok(sound_data) => { println!("sound data located: {:?}", sound_data.name) },
        Err(e) => { println!("error: {:?}", e) },
    }
}

fn main() {
    // Found sound
    match_sound(get_sound("alert"));
    // Missing sound
    match_sound(get_sound(""));
}
