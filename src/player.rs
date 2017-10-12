// #[macro_use]
// extern crate lazy_static;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use self::models::{Video, Room};
use std::{thread, time};
use schema;
use models;
use std::time::SystemTime;
use std::collections::HashMap;
use std::sync::Mutex;

use establish_connection;

lazy_static! {
    static ref PLAYLIST_THREADS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}


/// Fetches the current video from the playlist and waits for the duration of the video
/// Afterwards it updates the database and marks the video as played.
pub fn play_current_video<'a>(conn: &PgConnection, room_name: Option<String>) -> bool {
    use self::schema::videos::dsl::*;

    let video;
    match room_name.clone() {
        Some(room_name) => {
            video = videos
                .filter(played.eq(false))
                .filter(room.eq(room_name.to_lowercase()))
                .order(added_on)
                .first::<Video>(conn);
        },
        None => {
            video = videos
                .filter(played.eq(false))
                .filter(room.is_null())
                .order(added_on)
                .first::<Video>(conn);
        }
    };
    

    match video {
        Ok(video) => {
            let video_duration = time::Duration::from_secs(duration_to_seconds(&video.duration));

            super::diesel::update(&video)
                .set(played_on.eq(SystemTime::now()))
                .execute(conn)
                .expect("Unable to start playing the current video.");

            println!("Start playing: [{}] With ID: [{}] and duration: [{}] in room: [{:?}].", 
                &video.title, 
                &video.id, 
                &video.duration,
                room_name);

            // Wait until the video is played
            thread::sleep(video_duration);

            let now = SystemTime::now();
            // let elapsed = now.duration_since(played_on.unwrap());

            let mut playing: bool = true;

            // Continue playing this song while playing is true
            // Playing will be set to false if either the timer has run out
            // Or when someone skips the song by setting the PLAYLIST_THREADS[ROOM_NAME] to something other than "play"
            while playing {
                // Check if the video has ran out of time
                match now.elapsed() {
                    Ok(elapsed) => {
                        if elapsed.as_secs() >= video_duration.as_secs() {
                            playing = false;
                        }
                    },
                    Err(e) => {
                        playing = false;
                        println!("SystemTime elapsed error: {}", e);
                    }
                }

                let thread_name;
                match room_name.clone() {
                    Some(room_name) => {
                        thread_name = room_name;
                    },
                    None => {
                        thread_name = "".to_string();
                    }
                }
                // Check if someone tried to skip the video
                match PLAYLIST_THREADS.lock().unwrap().get(&thread_name) {
                    Some(thread_name) => {
                        if &thread_name[..] != "play" {
                            playing = false;
                        }

                    },
                    None => {
                        PLAYLIST_THREADS.lock().unwrap().insert(thread_name, "play".to_string());
                    }
                }

                thread::sleep(time::Duration::from_millis(500));
            }

            println!("Done playing [{}] from room [{:?}]", &video.title, room_name);

            // Mark the video as played
            super::diesel::update(&video)
                .set(played.eq(true))
                .execute(conn)
                .expect("Unable to mark the current video as played.");

            return true
        },
        Err(_) => return false,
    };
}


/// Start a thread to watch a certain playlist
pub fn play_video_thread<'a>(room: Option<String>) {

    let thread_name;

    match room.clone() {
        Some(room) => {
            thread_name = room;
        },
        None => {
            thread_name = "".to_string();
        }
    }

    thread::Builder::new()
        .name(thread_name)
        .spawn(move || {
            let mut result;
            let c = establish_connection();

            println!("Room name: {:?}", room.clone());
            loop {
                result = play_current_video(&c, room.clone());

                if ! result {
                    // Wait 1 second before trying to play a new video
                    thread::sleep(time::Duration::from_secs(1));
                }
            }
        })
        .unwrap();
}


/// Loop through every room & start playing their playlists
/// At the end of the loop, start the FFA playlist(room None)
pub fn init_playlist_listener<'a>() {
    use self::schema::rooms::dsl::*;

    let conn = establish_connection();

    let result = rooms.load::<Room>(&conn)
                .expect("Error loading videos");

    for room in result {
        PLAYLIST_THREADS.lock().unwrap().insert(room.name.clone(),"play".to_string());
        play_video_thread(Some(room.name));
    }

    // Also play the FFA room
    PLAYLIST_THREADS.lock().unwrap().insert("".to_string(),"play".to_string());
    play_video_thread(None);
}

/// Returns a duration string as seconds
/// EG: "PT1H10M10S" -> 4210
pub fn duration_to_seconds(duration: &String) -> u64 {
    let v: Vec<&str> = duration.split(|c: char| !c.is_numeric()).collect();
    let mut index: u32 = 0;
    let mut tmp: i32 = 0;

    for i in (0..v.len()).rev() {
        if ! v[i].is_empty() {
            tmp += v[i].parse::<i32>().unwrap() * (60i32.pow(index));
            index += 1;
        }
    }

    return tmp as u64
}


pub fn skip_video(room: Option<String>) {

    let room_name;

    match room.clone() {
        Some(room) => {
            room_name = room;
        },
        None => {
            room_name = "".to_string();
        }
    }

    let mut map = PLAYLIST_THREADS.lock().unwrap();

    println!("Skipping a song in room [{}]", room_name);

    if let Some(mut_key) = map.get_mut(&room_name) {
        *mut_key = "skip".to_string();
    }
}