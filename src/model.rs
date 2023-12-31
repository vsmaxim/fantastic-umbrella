use std::fs::{File, self};
use std::sync::{Arc, RwLock};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use oapi::{OApi, OApiOperation};
use sppparse::{SparsePointer, SparseRoot};

use crate::paths::Paths;

#[derive(Serialize, Deserialize, Clone)]
pub struct PathParam {
    pub name: String,
    pub param_type: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Request {
    pub method: String,
    pub title: String,
    pub url: String,
    pub body: String,
    pub query_params: Vec<String>,
    pub path_params: Vec<PathParam>,
}

impl Request {
    pub fn new_default() -> Request {
        Request {
            method: String::new(),
            title: String::new(),
            url: String::new(),
            body: String::new(),
            query_params: Vec::new(),
            path_params: Vec::new(),
        }
    }
    pub fn new(method: &str, title: &str, url: &str, body: &str) -> Request {
        Request {
            method: method.to_string(),
            title: title.to_string(),
            url: url.to_string(),
            body: body.to_string(),
            query_params: Vec::new(),
            path_params: Vec::new(),
        }
    }
}

impl From<&Request> for Request {
    fn from(v: &Self) -> Self {
        Request {
            method: String::from(&v.method),
            title: String::from(&v.title),
            url: String::from(&v.url),
            body: String::from(&v.body),
            query_params: vec![],
            path_params: vec![],
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
        let save_path = Paths::config_path();

        let file = File::create(save_path).expect("Couldn't create config file");

        // TODO: Avoid copying perhaps?
        let serializable: Vec<Request> = read.iter()
            .map(|r| Request::from(r.as_ref()))
            .collect();

        serde_json::to_writer_pretty(file, &serializable)
            .expect("Couldn't write file");
    }

    pub fn make_request(&self, request: &Request) {}

    pub fn import_from_openapi(spec_path: PathBuf) -> Self {
        let sparse_root = SparseRoot::new_from_file(spec_path).expect("Failed to parse the openapi");

        let doc: OApi = OApi::new(sparse_root);
        let root = doc.root_get().unwrap();
        let mut requests = Vec::new();

        let mut base_url = &"".to_string();

        match root.servers() {
            None => {}
            Some(servers) => {
                for s in servers.iter() {
                    base_url = s.url();
                }
            }
        }

        for (key, val) in root.paths().iter() {
            for (method, operation) in [
                ("GET", val.get()),
                ("POST", val.post()),
                ("PUT", val.put()),
            ].iter().filter_map(|&(m, v)| v.clone().map(|op| (m, op))) {
                let mut request = Request::new_default();
                request.method = method.to_string();
                request.title = operation.summary().clone().unwrap_or_default();

                for par in operation.clone().parameters().iter() {
                    match par.get() {
                        Ok(result) => {
                            request.query_params.push(result.name().clone());
                        }
                        Err(_) => {}
                    }
                }



                requests.push(request);
            }
        }

        return Self::new(requests);
    }

    pub fn load_from_disk_or_default() -> Self {
        let config_path = Paths::config_path();
        let openapi_spec_path = Paths::openapi_path();

        if config_path.exists() {
            let content = fs::read_to_string(config_path)
                .expect("Couldn't read file");

            Self::new(serde_json::from_str(&content).expect("Couldn't parse"))
        } else if openapi_spec_path.exists() {
            Self::import_from_openapi(openapi_spec_path)
        } else {
            let req = Request {
                method: "POST".into(),
                title: "Create request".into(),
                url: "http://google.com".into(),
                body: "hellooo".into(),
                query_params: vec![],
                path_params: vec![],
            };

            let ret = Self::new(vec![
                Request {
                    method: "POST".into(),
                    title: "Create request".into(),
                    url: "http://google.com".into(),
                    body: serde_json::to_string_pretty(&req).unwrap().into(),
                    query_params: vec![],
                    path_params: vec![],
                }
            ]);
            ret.save_on_disk();
            ret
        }
    }
}
