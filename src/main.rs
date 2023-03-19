use csv::Writer;
mod forest;
use clap::Parser;
mod webdev;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long)]
   forest: String,

   #[arg(short, long)]
   user: String,

   #[arg(short, long)]
   password: String,

   #[arg(short, long)]
   filepath: String,
}

fn write_time(path: &str, database_id: &str, notion_token: &str, time: String) {
  let mut wtr =Writer::from_path(path).unwrap();
  wtr.write_record(&["notion_token".to_string(), "database_id".to_string(), "time".to_string()]).unwrap();
  wtr.write_record(&[notion_token.to_string(), database_id.to_string(), time]).unwrap();
  wtr.flush().unwrap();
}

fn main() {
  let args = Args::parse();

  // 请求数据
  let (data, time) = forest::get_data_from_forest("".to_string(), args.forest);

  // // 写入time
  // write_time("./config/config.csv", &config.database_id, &config.notion_token, time)

}