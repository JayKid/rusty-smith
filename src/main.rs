mod parser;
use parser::get_posts;

fn main() {
    let posts = get_posts();
    for post in posts {
        println!("{:?}\n", post);
    }
}
