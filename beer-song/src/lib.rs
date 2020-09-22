fn full_song() -> Vec<String> {
    let mut song = Vec::new();
    for i in 0..100 {
        match i {
            0 => {
                let msg = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
                song.push(msg);
            },
            1 => {
                let msg = "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string();
                song.push(msg);
            },
            2 => {
                let msg = "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string();
                song.push(msg);
            },
            _ => {
                let msg = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", i, i, i-1);
                song.push(msg);
            }
        }
    }
    song
}

pub fn verse(n: usize) -> String {
    let result: String = full_song()[n].clone();
    result
}

pub fn sing(start: usize, end: usize) -> String {
    let mut song: Vec<String> = (&full_song()[end..start+1]).to_vec();
    song.reverse();
    let result = song.join("\n");
    result
}
