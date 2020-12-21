use structopt::StructOpt;
use std::io::{BufWriter, Write};
use std::fs::{create_dir_all, OpenOptions};
use chrono::{Utc, Datelike};

#[derive(StructOpt, Debug)]
#[structopt(name = "diary")]
struct Opt {
    #[structopt(subcommand)]
    sub: Sub
}

#[derive(StructOpt, Debug)]
enum Sub {
    #[structopt(name = "new", about = "Create a new diary directory")]
    New,

    #[structopt(name = "add", about = "Add a new page")]
    Add,
}


fn main() {
    let opt = Opt::from_args();
    let today = Utc::today();

    match &opt.sub {
        Sub::Add => {
            let dir_path = &format!("{}/{}", today.year(), today.month());
            if let Err(error) = create_dir_all(dir_path) {
                eprintln!("create_dir_all: {}", error);
                return;
            }

            let file_path = format!("{}/{}_{}_{}.md", dir_path, today.year(), today.month(), today.day());
            let file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(file_path)
                .unwrap();

            let mut f = BufWriter::new(file);
            let content = format!(
r#"# {}

## タグ

## やりたいこと

## できたこと

## メモ
"#
                                  , today.format("%Y/%m/%d"));
            if let Err(error) = f.write(content.as_bytes()) {
                eprintln!("f.write(content): {}", error);
            }
        },
        Sub::New => {
            if let Err(error) = create_dir_all("diary") {
                eprintln!("create_dir_all: {}", error);
                return;
            }

            let file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open("diary/README.md")
                .unwrap();

            let mut f = BufWriter::new(file);
            if let Err(error) = f.write("# README \n".as_bytes()) {
                eprintln!("f.write(content): {}", error);
            }

            let file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open("diary/TODO.md")
                .unwrap();

            let mut f = BufWriter::new(file);
            if let Err(error) = f.write("# TODO (やるべきこと)\n".as_bytes()) {
                eprintln!("f.write(content): {}", error);
            }

            let file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open("diary/CHANGELOG.md")
                .unwrap();

            let mut f = BufWriter::new(file);
            if let Err(error) = f.write("# CHANGELOG (実績)\n".as_bytes()) {
                eprintln!("f.write(content): {}", error);
            }

        },
    };

}
