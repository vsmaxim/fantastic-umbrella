use std::fs::{File, self};
use std::sync::{Arc, RwLock};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use oapi::{OApi};
use sppparse::{SparsePointer, SparseRoot};


const CONFIG_PATH: &str = "openapi/petstore.yaml";

#[derive(Serialize, Deserialize, Clone)]
pub struct Request {
    pub method: String,
    pub title: String,
    pub url: String,
    pub body: String,
    pub query_params: Vec<String>,
}


impl From<&Request> for Request {
    fn from(v: &Self) -> Self {
        Request {
            method: String::from(&v.method),
            title: String::from(&v.title),
            url: String::from(&v.url),
            body: String::from(&v.body),
            query_params: vec![],
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
        drop(write);
        self.save_on_disk();
    }

    pub fn update_request(&mut self, i: usize, r: &Request) {
        let lock_clone = self.requests.clone();
        let mut write = lock_clone.write().unwrap();
        write[i] = Arc::new(Request::from(r));
        drop(write);
        self.save_on_disk();
    }

    pub fn save_on_disk(&self) {
        let lock_clone = self.requests.clone();
        let read = lock_clone.read().unwrap();

        let file = File::create(&CONFIG_PATH)
            .expect("Couldn't create config file");

        // TODO: Avoid copying perhaps?
        let serializable: Vec<Request> = read.iter()
            .map(|r| Request::from(r.as_ref()))
            .collect();

        serde_json::to_writer_pretty(file, &serializable)
            .expect("Couldn't write file");
    }

    pub fn make_request(&self, request: &Request) {}

    pub fn load_from_disk_or_default() -> Self {
        if Path::new(CONFIG_PATH).exists() {
            if CONFIG_PATH.ends_with(".yaml") || CONFIG_PATH.ends_with(".yml") {
                let doc: OApi = OApi::new(
                    SparseRoot::new_from_file(PathBuf::from(CONFIG_PATH))
                        .expect("Failed to parse the openapi"),
                );
                let mut ret = Vec::new();
                let root = doc.root_get().unwrap();

                let mut base_url = &"".to_string();
                match root.servers() {
                    None => {}
                    Some(servers) => {
                        for (s) in servers.iter() {
                            base_url = s.url();
                        }
                    }
                }

                for (key, val) in root.paths().iter() {
                    let mut request = Request {
                        method: "".to_string(),
                        title: "".to_string(),
                        url: format!("{}{}", base_url, key),
                        body: "".to_string(),
                        query_params: vec![],
                    };

                    let mut oapi_operation;

                    match val.get() {
                        Some(value) => {
                            request.method = "GET".to_string();
                            oapi_operation = value;

                            for par in oapi_operation.parameters().iter() {
                                match par.get() {
                                    Ok(result) => {
                                        request.query_params.push(result.name().clone());
                                    }
                                    Err(_) => {}
                                }
                            }
                        }
                        _ => {}
                    }

                    match val.post() {
                        Some(value) => {
                            request.method = "POST".to_string();
                            oapi_operation = value;
                        }
                        _ => {}
                    }
                    match val.put() {
                        Some(value) => {
                            request.method = "PUT".to_string();
                            oapi_operation = value;
                        }
                        _ => {}
                    }

                    ret.push(request);
                }

                return Self::new(ret);
            }

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
                query_params: vec![],
            };

            let ret = Self::new(vec![
                Request {
                    method: "POST".into(),
                    title: "Create request".into(),
                    url: "http://google.com".into(),
                    body: serde_json::to_string_pretty(&req).unwrap().into(),
                    query_params: vec![],
                }
            ]);
            ret.save_on_disk();
            ret
        }
    }
}
