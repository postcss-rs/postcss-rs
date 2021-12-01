use std::process::Command;

#[allow(dead_code)]
struct Stats {
  slowest: f64,
  median: f64,
  fastest: f64,
  mean: f64,
}

const BENCH_LIST: [&str; 2] = ["benchmark_tokenizer", "benchmark_parser"];

const FILE_LIST: [(&str, &str); 6] = [
  ("tailwind-components.css", "2.8K"),
  ("bootstrap-reboot.css", "7.4K"),
  ("bootstrap-grid.css", "71K"),
  ("bootstrap.css", "201K"),
  ("tailwind.css", "3.5M"),
  ("tailwind-dark.css", "5.8M"),
];

fn main() {
  for bench in BENCH_LIST {
    for (file, size) in FILE_LIST {
      let kind = format!("{}/{}({}):", &bench[10..], file, size);

      let program = "node";
      let js_file = format!("adapter/{}.js", bench);
      let args: Vec<&str> = vec![&js_file, file];
      let js = exec(program, args);
      println!(
        "js  : {:40} {:>6.2}, {:>6.2}, {:>6.2}",
        kind, js.fastest, js.mean, js.slowest,
      );

      let program = format!("target/release/{}", bench);
      let args = vec![file];
      let rust = exec(&program, args);
      println!(
        "rust: {:40} {:>6.2}, {:>6.2}, {:>6.2}",
        kind, rust.fastest, rust.mean, rust.slowest,
      );

      println!("{:55} {:>5.0}x", "", js.mean / rust.mean,);
    }
  }
}

fn exec(program: &str, args: Vec<&str>) -> Stats {
  let mut times: Vec<u128> = vec![];

  const COUNT: usize = 10;
  for _ in 0..=COUNT {
    let output = Command::new(program)
      .args(&args)
      .output()
      .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
      let s = String::from_utf8_lossy(&output.stdout);
      times.push(s.trim_end().parse().unwrap());
    } else {
      let s = String::from_utf8_lossy(&output.stderr);
      panic!("{} failed and stderr was:\n{}", program, s);
    }
  }

  times.sort_unstable();

  Stats {
    slowest: times[0] as f64 / 1_000_000f64,
    median: times[COUNT / 2] as f64 / 1_000_000f64,
    fastest: times[COUNT - 1] as f64 / 1_000_000f64,
    mean: times.iter().map(|&v| v as f64).sum::<f64>() / COUNT as f64 / 1_000_000f64,
  }
}
