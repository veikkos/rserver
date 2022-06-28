use rouille::input;
use rouille::Request;
use rouille::Response;
use std::fs;
use std::path::Path;

const PORT: i32 = 80;

fn get_asset_path(file: String) -> String {
    format!("./assets/{}", file)
}

fn get_html_response(file: String) -> Response {
    println!("html - {}", file);
    let contents = fs::read_to_string(get_asset_path(file));

    let contents = match contents {
        Ok(file) => file,
        Err(_error) => return Response::empty_404()
    };

    Response::html(&contents)
}

fn get_binary_response(file: String) -> Response {
    println!("bin - {}", file);
    let contents = fs::read(get_asset_path(file));

    let contents = match contents {
        Ok(file) => file,
        Err(_error) => return Response::empty_404()
    };

    Response::from_data("application/octet-stream", contents)
}

fn handle_after_login(request: &Request) -> Response {
    let url = request.url();
    let file = match url.as_str(){
        "/" => "index.html".to_owned(),
        _ => (&url[1..url.len()]).to_string(),
    };
    let ext = Path::new(&file).extension();
    match ext {
        Some(x) => match x.to_str().unwrap() {
            "html" => return get_html_response(file),
            _ => return get_binary_response(file)
        },
        None => return Response::empty_400(),
    };
}

fn main() {
    println!("Now listening on {}", PORT);
    let addr = format!("0.0.0.0:{}", PORT);

    rouille::start_server(addr, move |request| {
        let auth = match input::basic_http_auth(request) {
            Some(a) => a,
            None => return Response::basic_http_auth_login_required("realm"),
        };

        if auth.login == "user" && auth.password == "1234" {
            handle_after_login(request)
        } else {
            Response::text("Bad login/password").with_status_code(403)
        }
    });
}
