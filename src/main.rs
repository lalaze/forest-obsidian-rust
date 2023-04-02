mod forest;
use clap::Parser;
mod webdav;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long)]
   forest: String,

   #[arg(short, long)]
   user: String,

   #[arg(short, long)]
   password: String,

   #[arg(long)]
   filepath: String,
}

fn main() {
  let args = Args::parse();

  // 请求数据
  let data = forest::get_data_from_forest(args.forest);

  webdav::to_serve(args.user, args.password, args.filepath, data);

}