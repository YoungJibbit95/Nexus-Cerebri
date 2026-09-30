// Disposable, std-only test process. Never included in a production Cargo target.
use std::{
    env, fs,
    io::{self, Read, Write},
    thread,
    time::Duration,
};

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let path = env::current_exe().unwrap();
    let mode = path.file_stem().unwrap().to_str().unwrap();
    match mode {
        "hang" => {
            fs::write(path.with_extension("pid"), std::process::id().to_string()).unwrap();
            loop {
                thread::sleep(Duration::from_secs(1));
            }
        }
        "oversize" => {
            let chunk = vec![b'x'; 65536];
            for _ in 0..257 {
                if io::stdout().write_all(&chunk).is_err() {
                    return;
                }
            }
        }
        "malformed" => {
            println!("private-output-that-must-not-escape");
        }
        "failure" => {
            eprintln!("private-stderr-that-must-not-escape");
            std::process::exit(1);
        }
        "incompatible" => println!("{{\"integration_version\":{{\"major\":99,\"minor\":0}}}}"),
        "mismatch" => println!(
            "{{\"integration_version\":{{\"major\":0,\"minor\":1}},\"status\":\"planned\",\"request_id\":\"other-request\",\"trace_id\":\"synthetic-trace\",\"context_revision\":1,\"result\":{{}}}}"
        ),
        _ => panic!("unknown test mode"),
    }
}
