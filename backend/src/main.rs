use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct GenerateRequest {
    input: String,
    preset: Option<String>,
    format: Option<String>, // "png" or "svg"
    size_px: Option<u32>,
    padding_frac: Option<f32>,
    variant: Option<u64>,
    transparent_background: Option<bool>,
}

fn header(k: &[u8], v: &[u8]) -> tiny_http::Header {
    tiny_http::Header::from_bytes(k, v).unwrap()
}

fn respond_with_cors(status: u16, content_type: &str, body: Vec<u8>) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let mut response = tiny_http::Response::from_data(body).with_status_code(status);
    response.add_header(header(b"Content-Type", content_type.as_bytes()));
    response.add_header(header(b"Access-Control-Allow-Origin", b"*"));
    response
}

fn respond_no_content() -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let mut response = tiny_http::Response::from_data(Vec::new()).with_status_code(204);
    response.add_header(header(b"Access-Control-Allow-Origin", b"*"));
    response.add_header(header(b"Access-Control-Allow-Methods", b"POST, OPTIONS"));
    response.add_header(header(b"Access-Control-Allow-Headers", b"content-type"));
    response
}

fn main() {
    // Attempt to load an owned font bytes from `assets/fonts/` at startup.
    let runtime_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/fonts/LiberationSans-Bold.ttf");
    let font: Option<Arc<Vec<u8>>> = match std::fs::read(&runtime_path) {
        Ok(bytes) => Some(Arc::new(bytes)),
        Err(_) => None,
    };

    let server = tiny_http::Server::http("0.0.0.0:3000").expect("failed to bind");
    println!("backend listening on http://0.0.0.0:3000");

    for mut request in server.incoming_requests() {
        let url = request.url().to_string();

        if request.method() == &tiny_http::Method::Options && url == "/generate" {
            let _ = request.respond(respond_no_content());
            continue;
        }

        if request.method() == &tiny_http::Method::Post && url == "/generate" {
            let mut content = String::new();
            if let Err(e) = request.as_reader().read_to_string(&mut content) {
                let _ = request.respond(respond_with_cors(400, "text/plain", format!("failed read body: {}", e).into_bytes()));
                continue;
            }

            let payload: GenerateRequest = match serde_json::from_str(&content) {
                Ok(p) => p,
                Err(e) => {
                    let _ = request.respond(respond_with_cors(400, "text/plain", format!("invalid json: {}", e).into_bytes()));
                    continue;
                }
            };

            // parse preset
            let preset = match payload.preset.as_deref().unwrap_or("monogram-badge").parse::<logo_gen::Preset>() {
                Ok(p) => p,
                Err(e) => {
                    let _ = request.respond(respond_with_cors(400, "text/plain", e.to_string().into_bytes()));
                    continue;
                }
            };

            let mut opts = logo_gen::RenderOptions::default();
            if let Some(size) = payload.size_px { opts.size_px = size; }
            if let Some(pad) = payload.padding_frac { opts.padding_frac = pad; }
            if let Some(v) = payload.variant { opts.variant = Some(v); }
            if let Some(t) = payload.transparent_background { opts.transparent_background = t; }

            let format = payload.format.unwrap_or_else(|| "png".to_string());

            // generate and respond
            let res = if format == "png" {
                let owned_font = font.as_ref().map(|a| (**a).clone());
                match logo_gen::LogoGenerator::generate_png_with_owned_font(&payload.input, preset, &opts, owned_font) {
                    Ok(bytes) => respond_with_cors(200, "image/png", bytes),
                    Err(e) => respond_with_cors(500, "text/plain", e.to_string().into_bytes()),
                }
            } else if format == "svg" {
                match logo_gen::LogoGenerator::generate_svg(&payload.input, preset, &opts) {
                    Ok(svg) => respond_with_cors(200, "image/svg+xml;charset=utf-8", svg.into_bytes()),
                    Err(e) => respond_with_cors(500, "text/plain", e.to_string().into_bytes()),
                }
            } else {
                respond_with_cors(400, "text/plain", format!("unsupported format: {}", format).into_bytes())
            };

            let _ = request.respond(res);
            continue;
        }

        // not found
        let _ = request.respond(respond_with_cors(404, "text/plain", b"not found".to_vec()));
    }


}
