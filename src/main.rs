mod parser;
mod builder;
use builder::build;
use parser::get_posts;

fn main() {
    let posts = get_posts();
    build(&posts);
}
