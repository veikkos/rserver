use rouille::input;
use rouille::Request;
use rouille::Response;

const PORT: i32 = 80;

fn handle_after_login(_request: &Request) -> Response {
    Response::text("You are in a secret area")
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
