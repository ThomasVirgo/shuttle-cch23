use actix_web::{get, web::Path, web::ServiceConfig, Error, HttpResponse};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/1/{params:.*}")]
async fn day1(path: Path<String>) -> Result<HttpResponse, Error> {
    let mut params_str = path.into_inner();
    if params_str.ends_with("/") {
        params_str.pop();
    }
    let params: Vec<&str> = params_str.split("/").collect();
    let length = params.len();
    println!("{:?}", params);
    let mut result: u32 = 0;
    if length == 1 {
        let num = params.first().clone().unwrap().parse::<u32>().unwrap();
        return Ok(HttpResponse::Ok().json(num.pow(3)));
    } else {
        for num_str in params {
            result = result ^ num_str.parse::<u32>().unwrap();
            println!("{}", result);
        }
    }
    result = result.pow(3);
    Ok(HttpResponse::Ok().json(result))
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(day1);
    };

    Ok(config.into())
}
