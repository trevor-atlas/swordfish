use crate::settings::AppConfig;
use ignore::DirEntry;
use ignore::WalkBuilder;
use std::cmp;
use std::io::Write;

pub fn search() {
    let (tx, rx) = crossbeam_channel::bounded::<DirEntry>(300);
    let settings = AppConfig::new();
    if let Some(dirs) = settings.get_search_directories() {
        if dirs.is_empty() {
            return;
        }
        let stdout_thread = std::thread::spawn(move || {
            let mut stdout = std::io::BufWriter::new(std::io::stdout());
            for dent in rx {
                stdout
                    .write(dent.path().to_string_lossy().as_bytes())
                    .unwrap();
                stdout.write(b"\n").unwrap();
            }
        });
        let mut walker = WalkBuilder::new(dirs[0].clone());
        dirs.iter()
            .skip(1)
            .fold(&mut walker, |builder, dir| builder.add(dir))
            .threads(cmp::min(12, num_cpus::get()))
            .build_parallel()
            .run(|| {
                let tx = tx.clone();
                Box::new(move |result| {
                    use ignore::WalkState::*;
                    match result {
                        Ok(dir) => tx.send(dir).unwrap(),
                        Err(e) => {
                            println!("error: {:?}", e);
                        }
                    };
                    Continue
                })
            });

        drop(tx);
        stdout_thread.join().unwrap();
        // let v: Vec<_> = rx.iter().collect();
        // v.iter()
        //     .for_each(|d| println!("{}", d.path().to_str().unwrap()));
    }
}
