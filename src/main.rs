use redis::Commands;

fn main() -> Result<(), redis::RedisError> {
    let args: Vec<_> = std::env::args().collect();
    match args.len() {
        2 => {}
        _ => {
            panic!("cargo run -- REDIS_URL");
        }
    }

    let redis = args[1].clone();

    let client = redis::Client::open(redis).expect("Unable to create redis client");
    let mut conn = client.get_connection().expect("Failed to connect to redis");

    for value in conn.scan::<String>()? {
        println!("{value}");
    }

    Ok(())
}
