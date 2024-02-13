use std::{path::{Path, PathBuf}, time::Duration};

use futures::executor;

use minio::s3::args::{BucketExistsArgs, MakeBucketArgs, UploadObjectArgs};
use minio::s3::client::Client;
// use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;

use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::new_debouncer;

const IMAGE_EXTENSIONS: [&str; 4] = ["png", "jpg", "jpeg", "bmp"];
const DEFAULT_OBJSTO_CLIENT_NAME: &str = "http://localhost:9000";
const DEFAULT_OBJSTO_BUCKET_NAME: &str = "wru-bucket";

#[tokio::main]
async fn main() {
    // set up debouncer
    let (tx, rx) = std::sync::mpsc::channel();

    // no specific tickrate, max debounce time 1 second
    let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx).unwrap();
    debouncer.watcher().watch(Path::new("."), RecursiveMode::Recursive).unwrap();

    // set up minio conn
    let base_url = DEFAULT_OBJSTO_CLIENT_NAME.parse::<BaseUrl>().unwrap_or_default();

    // TODO: get variables from args, incl. creds

    // let static_provider = StaticProvider::new(
    //     "Q3AM3UQ867SPQQA43P2F",
    //     "zuf+tfteSlswRu7BJ86wekitnifILbZam1KYY3TG",
    //     None,
    // );
    let client = Client::new(
        base_url.clone(),
        None,
        None,
        None,
    )
    .unwrap();

    let bucket_name = DEFAULT_OBJSTO_BUCKET_NAME;

    for result in rx {
        match result {
            Ok(events) => events.iter().for_each(|event| {
                if event.kind.is_create() {
                    event.paths.iter().for_each(|path| {
                        if is_an_image(path) {
                            // TODO: don't explode if MinIO server is down
                            let _ = executor::block_on(upload_object(&client, bucket_name, path));
                        }
                    });
                }
            }),
            Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
        }
    }
}

async fn upload_object(client: &Client, bucket_name: &str, path_buf: &PathBuf) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path_as_str = path_buf.to_str().unwrap_or_default();
    let file_name_in_bucket = path_as_str;
    // Check if bucket exists or not
    let exists = client
        .bucket_exists(&BucketExistsArgs::new(&bucket_name).unwrap())
        .await
        .unwrap();

    // Make bucket if doesn't exist
    if !exists {
        client
            .make_bucket(&MakeBucketArgs::new(&bucket_name).unwrap())
            .await
            .unwrap();
    }

    client
        .upload_object(
            &mut UploadObjectArgs::new(
                &bucket_name,
                file_name_in_bucket,
                path_as_str,
            )
            .unwrap(),
        )
        .await
        .unwrap();

    println!("{path_as_str} is successfully uploaded as object {file_name_in_bucket} to {bucket_name}.");
    Ok(())
}

fn is_an_image(path: &PathBuf) -> bool {
    let extension = path.extension().unwrap_or_default().to_str().unwrap_or_default();
    return IMAGE_EXTENSIONS.contains(&extension)
}
