use anyhow::{anyhow, Result};
use rouille::{input, match_assets, router, Request, Response, Server};
use std::process::exit;

use super::styles::Style;

pub fn start(
    style: &Style,
    address: &str,
    oneshot: bool,
    login: Option<String>,
    password: Option<String>,
) -> Result<()> {
    let rendered_template = style.render()?;
    let style_base_path = style
        .base_path
        .to_str()
        .ok_or_else(|| anyhow!("Could not convert style base path to str."))?
        .to_owned();

    let server = Server::new(address, move |request| {
        if login.is_some() && password.is_some() {
            if let Some(rep) =
                handle_auth(request, login.as_ref().unwrap(), password.as_ref().unwrap())
            {
                return rep;
            }
        }

        router!(request,
        (GET) (/) => {
            if let Some(command) = request.get_param("cmd") {
                handle_cmd(command)
            } else if let Some(entry) = request.get_param("select") {
                handle_select(entry, oneshot)
            } else {
                Response::html(&rendered_template)
            }
        },
        (GET) (/select/{entry: String}) => {
            handle_select(entry, oneshot)
        },
        (GET) (/cmd/{command: String}) => {
            handle_cmd(command)
        },
        _ => {
            let response = match_assets(request, &style_base_path);
            if response.is_success() {
                response
            } else {
                Response::empty_404()
            }
        }
        )
    })
    .unwrap();
    server.run();
    Ok(())
}

pub fn handle_select(entry: String, oneshot: bool) -> Response {
    println!("{}", entry);
    if oneshot {
        exit(0);
    }
    Response::redirect_302("/")
}

pub fn handle_cmd(command: String) -> Response {
    match command.as_str() {
        "shutdown" => {
            exit(0);
        }
        _ => Response::empty_404(),
    }
}

pub fn handle_auth(request: &Request, login: &str, password: &str) -> Option<Response> {
    let auth = match input::basic_http_auth(request) {
        Some(a) => a,
        None => return Some(Response::basic_http_auth_login_required("thqm")),
    };

    if auth.login != login || auth.password != password {
        Some(Response::text("Bad login/password").with_status_code(403))
    } else {
        None
    }
}
