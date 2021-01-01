use chrono::{DateTime, Utc};
use futures::future::join_all;
use hyper::{ client::HttpConnector, Client};
use hyper_tls::HttpsConnector;
use log::{error, info};
use std::{
    collections::HashMap,
    io::ErrorKind,
    path::PathBuf,
    sync::{Arc, Mutex},
    unimplemented,
};
use tokio::{
    fs::{create_dir},
    task::JoinHandle,
};

use crate::tree::{IlNode, IlNodeType};

struct FileInfo {
    ending: String,
    uri: String,
    date: DateTime<Utc>,
    path: PathBuf,
}

pub fn sync(
    node: &'static IlNode,
    mut path: PathBuf,
    client: Arc<Client<HttpsConnector<HttpConnector>>>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut sync_handles = vec![];
        match node.breed {
            IlNodeType::Folder => {
                path.push(&node.title);
                match create_dir(&path).await {
                    Ok(_) => {
                        info!("created Folder {}", &node.title)
                    }
                    Err(err) if err.kind() == ErrorKind::AlreadyExists => {}
                    Err(err) => {
                        error!("couldn't create folder \"{}\" - {}", &node.title, err)
                    }
                }
                if let Some(children) = &node.children {
                    for child in children
                        .iter()
                        .filter(|child| child.breed == IlNodeType::Folder)
                    {
                        sync_handles.push(sync(child, path.clone(), client.clone()));
                    }
                }
            }
            _ => (),
        }
        join_all(sync_handles).await;
    })
}

pub struct FileWatcher {
    files: Mutex<HashMap<String, FileInfo>>,
}

impl FileWatcher {
    pub fn add_file(&mut self, file_info: FileInfo) {
        self.files
            .lock()
            .unwrap()
            .insert(file_info.uri.to_string(), file_info);
    }
    fn download_file(_uri: &str, _path: &PathBuf, _ending: &str) -> JoinHandle<()> {
        tokio::spawn(async move { unimplemented!() })
    }
    pub fn new() -> Self {
        return {
            FileWatcher {
                files: Mutex::new(HashMap::new()),
            }
        };
    }
}