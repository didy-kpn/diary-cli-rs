use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "diary")]
struct Opt {
    #[structopt(subcommand)]
    sub: Sub,
}

#[derive(StructOpt, Debug)]
enum Sub {
    #[structopt(name = "new", about = "Create a new diary directory")]
    New,

    #[structopt(name = "add", about = "Add a new page")]
    Add(PageKind),
}

#[derive(StructOpt, Debug)]
enum PageKind {
    #[structopt(name = "entry", about = "Add today's entry")]
    Entry,

    #[structopt(name = "article", about = "Add new article")]
    Article,
}

#[derive(Debug, Serialize, Deserialize)]
struct DiarySetting {
    entry_path: String,
    article_path: String,
    diary_template: String,
}

fn main() {
    let opt = Opt::from_args();
    let today = Utc::today();
    let setting_file_name = "diary-cli.yaml";

    match &opt.sub {
        // ページを追加
        Sub::Add(add) => {
            // 設定ファイルを読み込む
            let file = OpenOptions::new().read(true).open(setting_file_name);
            if let Err(err) = file {
                eprintln!("diary-cli.yaml: {}", err);
                return;
            }
            let file = file.unwrap();
            let mut f = BufReader::new(file);
            let mut content = String::new();
            f.read_to_string(&mut content).unwrap();
            let setting: DiarySetting = serde_yaml::from_str(&content).unwrap();

            match add {
                // 本日の日記
                PageKind::Entry => {
                    let dir_path =
                        &format!("{}/{}/{}", setting.entry_path, today.year(), today.month());
                    if !Path::new(dir_path).exists() {
                        if let Err(error) = create_dir_all(dir_path) {
                            eprintln!("create_dir_all: {}", error);
                            return;
                        }
                    }

                    let file_name =
                        format!("{}_{}_{}.md", today.year(), today.month(), today.day());
                    let content = setting
                        .diary_template
                        .replace("{}", &today.format("%Y/%m/%d").to_string());
                    write_file(dir_path, &file_name, &content);
                }
                // まとめなどの記事
                PageKind::Article => {}
            }
        }
        // 日記を作成する
        Sub::New => {
            let directory_path = "diary";

            // 日記ディレクトリまたは設定ファイルが存在してる場合はディレクトリを作成しない
            if Path::new(directory_path).exists() || Path::new(setting_file_name).exists() {
                eprintln!("not ok: diary or diary-cli.yaml exists");
                return;
            }

            if let Err(error) = create_dir_all(directory_path) {
                eprintln!("create_dir_all: {}", error);
                return;
            } else {
                println!("ok: diary directory");
            }

            // 各記事へのリンク(README)
            write_file(directory_path, "README.md", "# README \n");

            // 今後のやるべきことリスト(TODO)
            write_file(directory_path, "TODO.md", "# TODO (やるべきこと)\n");

            // 実績(CHANGELOG)
            write_file(directory_path, "CHANGELOG.md", "# CHANGELOG (実績)\n");

            // ガイドライン(CONTRIBUTING)
            write_file(directory_path, "CONTRIBUTING.md", "# CONTRIBUTING (ガイドライン)\n");

            // 設定ファイル
            let diary_template = r#"# {}

## Concrete Experience (具体的経験)

## Reflective Observation (省察)

## Abstract Conceptualization (概念化):

## Active Experimentation (試行):

"#;
            let setting = DiarySetting {
                entry_path: format!(
                    "{}/{}/entries",
                    env::current_dir().unwrap().display(),
                    directory_path
                ),
                article_path: format!(
                    "{}/{}/articles",
                    env::current_dir().unwrap().display(),
                    directory_path
                ),
                diary_template: diary_template.to_string(),
            };
            write_file(
                directory_path,
                setting_file_name,
                &serde_yaml::to_string(&setting).unwrap(),
            );
        }
    };
}

fn write_file(directory_path: &str, file_name: &str, content: &str) {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("{}/{}", directory_path, file_name))
        .unwrap();

    let mut f = BufWriter::new(file);
    if let Err(error) = f.write(content.as_bytes()) {
        eprintln!("f.write(content): {}", error);
    } else {
        println!("ok: {}/{} file", directory_path, file_name);
    }
}
