use std::fs::{File, self};
use std::sync::{Arc, RwLock};
use std::path::Path;
use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = "config.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct Request {
    pub method: String,
    pub title: String,
    pub url: String,
    pub body: String,
}


impl From<&Request> for Request {
    fn from(v: &Self) -> Self {
        Request { 
            method: String::from(&v.method),
            title: String::from(&v.title),
            url: String::from(&v.url),
            body: String::from(&v.body),
        }
    }
}


pub struct Model {
    pub requests: Arc<RwLock<Vec<Arc<Request>>>>,
}

impl Model {
    pub fn new(requests: Vec<Request>) -> Self {
        let arc_r: Vec<Arc<Request>> = requests
            .iter()
            .map(|r| Arc::new(Request::from(r)))
            .collect();

        Self { requests: Arc::new(RwLock::new(arc_r)) }
    }

    pub fn add_request(&mut self, r: Request) { 
        let lock_clone = self.requests.clone();
        let mut write = lock_clone.write().unwrap();
        write.push(Arc::new(r));
        self.save_on_disk();
    }

    pub fn update_request(&mut self, i: usize, r: Request) {
        let lock_clone = self.requests.clone();
        let mut write = lock_clone.write().unwrap();
        write[i] = Arc::new(r); 
        self.save_on_disk();
    }

    pub fn save_on_disk(&self) {
        let file = File::create(&CONFIG_PATH)
            .expect("Couldn't create config file");

        let lock_clone = self.requests.clone();
        let read = lock_clone.read().unwrap();

        // TODO: Avoid copying perhaps?
        let serializable: Vec<Request> = read.iter()
            .map(|r| Request::from(r.as_ref()))
            .collect();

        serde_json::to_writer_pretty(file, &serializable)
            .expect("Couldn't write file");
    }

    pub fn load_from_disk_or_default() -> Self {
        if Path::new(CONFIG_PATH).exists() {
            let content = fs::read_to_string(CONFIG_PATH)
                .expect("Couldn't read file");

            Self::new(
                serde_json::from_str(&content)
                .expect("Couldn't parse")
            )
        } else {
            let req = Request {
                method: "POST".into(),
                title: "Create request".into(),
                url: "http://google.com".into(),
                body: "hellooo".into(),
            };

            let ret = Self::new(vec![
                Request {
                    method: "POST".into(),
                    title: "Create request".into(),
                    url: "http://google.com".into(),
                    body: serde_json::to_string_pretty(&req).unwrap().into(),
                }
            ]);
            ret.save_on_disk();
            ret
        }
    }
}
