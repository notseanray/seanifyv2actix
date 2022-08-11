use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, RwLock};
use std::{collections::VecDeque, fs};
use actix_web::HttpResponse;
use actix_web::http::{header::ContentType, StatusCode};
use actix_web::{get, web, App, HttpServer, Responder};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub(crate) static ref SONG_LIST: Arc<RwLock<HashMap<String, SongInMemory>>> = Arc::new(RwLock::new(HashMap::new()));
}

// Keep list of blocked ips, we store them seperately so it's quicker to access since we don't need
// to load the total list of ips
#[derive(Default)]
struct BlockedList {
    list: BTreeMap<u64, usize>,
}

/*
 * Keep a list of ips that have connected and cycle through them every set amount of ms, if there
 * are too many instances of the same ip in the list at the same time we add them to the blocked
 * list
 *
 * Since the blockedlist needs to be read everytime there is a new connection we only store a
 * reference to it in this struct so we can avoid locking both the ratelimiter and blockedlist at
 * the same time
 *
 * Since the blocked list is going to be read a lot more often we keep it in an RwLock instead of a
 * mutex
 */
struct RateLimiter<'a> {
    username_list: VecDeque<u64>,
    blocked_list: &'a Arc<std::sync::RwLock<BlockedList>>,
}

impl<'a> RateLimiter<'a> {
    pub fn new(blocked_list: &'a Arc<std::sync::RwLock<BlockedList>>) -> Self {
        Self {
            username_list: VecDeque::with_capacity(1),
            blocked_list,
        }
    }

    /*
    pub fn add(&mut self, username: u64) {
        if self.username_list.len() < MAX_CLIENT_RATE_CACHE {
            self.username_list.push_back(username);
        }
    }

    pub fn cycle(&mut self) {
        let _ = self.username_list.pop_front();
        self.check_if_limited();
    }

    fn check_if_limited(&mut self) {
        let mut usernames = BTreeMap::new();
        for username in self.username_list.iter() {
            let count = usernames.entry(username).or_insert(0);
            *count += 1;
        }
        for username in usernames.iter() {
            if let Some(v) = usernames.get(username.0) {
                if v > &*MAX_RATELIMIT {
                    let mut locked = self.blocked_list.write().unwrap();
                    // "**" lmao wtf
                    locked.list.insert(**username.0, RATE_BAN_IN_SECONDS);
                }
            }
        }
    }
    */
}

pub(crate) struct YtSong {}


pub(crate) struct Config {
    max_ratelimit: usize,
    max_client_rate_cache: usize,
    port: u16,
    admin_key: String,
    max_songs: usize,
    max_connections: usize,
    max_song_folder_size_gb: usize,
    ban_time_sec: usize,
    retries: usize,
    yt_timeout_ms: usize,
}

pub(crate) struct User {
    username: String,
    password: String,
    public: bool,
    last_played: VecDeque<String>,
    display_name: String,
    followers: Vec<String>,
    following: Vec<String>,
    analytics: bool,
    admin: bool,
}

pub(crate) struct Song {
    id: String,
    title: String,
    album: Option<String>,
    artist: String,
    duration: f64,
    genre: Option<String>,
    track_disc: [u16; 2],
    album_arist: Vec<String>,
    size: u64,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct SongInMemory {
    title: String,
    album: Option<String>,
    duration: f64,
}

impl Song {}

#[get("/song_list")]
async fn song_list(_: HttpResponse) -> impl Responder {
    let data = fs::read_to_string("./music_list.json").unwrap_or_default();
    let json_data: Vec<SongInMemory> = serde_json::from_str(&data).unwrap();
    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .body(serde_json::to_string(&json_data).unwrap_or_else(|_| "[]".to_string()))
}

#[get("/song/{name}")]
async fn song_data(name: web::Path<String>) -> impl Responder {
    let data = fs::read_to_string(format!("./music_data/{}", name)).unwrap_or_default();
    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .body(serde_json::to_string(&data).unwrap_or_default())
}

pub(crate) async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new().service(song_data)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
