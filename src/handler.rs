use rouille::Request;
use rouille::Response;
use std::fs;
use std::path::Path;

fn get_asset_path(file: String) -> String {
    format!("./assets/{}", file)
}

fn get_html_response(file: String) -> Response {
    println!("html - {}", file);

    let contents = match fs::read_to_string(get_asset_path(file)) {
        Ok(file) => file,
        Err(_error) => return Response::empty_404()
    };

    Response::html(&contents)
}

fn get_binary_response(file: String) -> Response {
    println!("bin - {}", file);

    let contents = match fs::read(get_asset_path(file)) {
        Ok(file) => file,
        Err(_error) => return Response::empty_404()
    };

    Response::from_data("application/octet-stream", contents)
}

fn get_file_name(url: String) -> String {
    match url.as_str(){
        "/" => "index.html".to_owned(),
        _ => (&url[1..url.len()]).to_string(),
    }
}

pub fn request(request: &Request) -> Response {
    let url = request.url();
    let file = get_file_name(url);
    match Path::new(&file).extension() {
        Some(x) => match x.to_str().unwrap() {
            "html" => return get_html_response(file),
            &_ => return get_binary_response(file)
        },
        None => return Response::empty_400(),
    };
}
