mod request;
mod restore;
use std::fs;

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

        let image_url = format!("https://cdn-msp3.18comic.vip/media/photos/{}/{}.webp", aid_str, pid_str.as_str());
        let image_data = request::download(image_url.as_str()).unwrap();

        if image_data.is_empty() {
            break;
        }

        let image_path = format!("{}/{}.webp", aid_str, pid_str.as_str());
        fs::write(image_path.as_str(), image_data).unwrap();

        let restore_image_path = format!("{}/{}.jpg", aid_str, pid_str);
        restore::restore_image(image_path.as_str(), restore_image_path.as_str(), aid_str, pid_str.as_str());
    }
}

fn main() {
    download_jm_comics("404551");
}
