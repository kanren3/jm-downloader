mod request;
mod restore;
use std::fs;
use std::thread;

fn get_jm_comics_title(aid_str: &str) -> String {
    let title_url = format!("https://18comic.vip/album/{}", aid_str);
    let title_data = request::download(title_url.as_str()).unwrap();
    let title = String::from_utf8_lossy(&title_data).to_string();

    let start_index = title.find("<title>").unwrap() + 7;
    let end_index = title.find("</title>").unwrap();
    let title = &title[start_index..end_index];

    title.to_string()
}

fn download_jm_comics(aid_str: &str) {
    fs::create_dir_all(aid_str).unwrap();

    let cover_url = format!("https://cdn-msp3.18comic.vip/media/albums/{}.jpg", aid_str);
    let cover_data = request::download(cover_url.as_str()).unwrap();

    let cover_path = format!("{}/cover.jpg", aid_str);
    fs::write(cover_path, cover_data).unwrap();

    let mut pid = 0;

    loop {
        pid += 1;
        let pid_str = format!("{:05}", pid);

        let image_url = format!(
            "https://cdn-msp3.18comic.vip/media/photos/{}/{}.webp",
            aid_str,
            pid_str.as_str()
        );
        let image_data = request::download(image_url.as_str()).unwrap();

        if image_data.is_empty() {
            break;
        }

        let image_path = format!("{}/{}.webp", aid_str, pid_str.as_str());
        fs::write(image_path.as_str(), image_data).unwrap();

        let restore_image_path = format!("{}/{}.jpg", aid_str, pid_str);
        restore::restore_image(
            image_path.as_str(),
            restore_image_path.as_str(),
            aid_str,
            pid_str.as_str(),
        );
    }

    let title = get_jm_comics_title(aid_str);
    let metadata_path = format!("{}/metadata.json", aid_str);
    let metadata = format!(
        r#"{{"title": "{}", "number_of_pages": {}}}"#,
        title,
        pid - 1
    );
    fs::write(metadata_path, metadata).unwrap();
}

static COMICS_ARRAY: &[&str] = &["1019260", "1020201"];

fn main() {
    let mut thread_count = 10;
    let mut thread_handles: Vec<std::thread::JoinHandle<()>> = vec![];

    if thread_count > COMICS_ARRAY.len() {
        thread_count = COMICS_ARRAY.len();
    }

    let comics_chunks = COMICS_ARRAY.chunks(COMICS_ARRAY.len() / thread_count);

    for chunk in comics_chunks {
        let comics_vec = chunk.to_vec();
        let handle = thread::spawn(move || {
            for aid_str in comics_vec {
                println!("Downloading comics: {}", aid_str);
                download_jm_comics(aid_str);
            }
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
