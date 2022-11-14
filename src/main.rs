use std::env;
use std::thread;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    // TODO: 引数なかった時のエラーハンドリング
    let args: Vec<String> = env::args().collect();
    let arg: u64 = args[1].parse().unwrap();

    let pb = ProgressBar::new(arg);
    // TODO: 経過時間はいらないかも
    let sty = ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue}")
        .unwrap()
        .progress_chars("##-");

    for _ in 0..arg {
        pb.inc(1);
        pb.set_style(sty.clone());
        thread::sleep(Duration::from_secs(1));
    }
}
