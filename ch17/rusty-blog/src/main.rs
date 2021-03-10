use rusty_blog::Post;

fn main() {
	// as a result of changes, now need more shadowing
	let mut post = Post::new();

	post.add_text("I ate a salad for lunch today.");

	let post = post.request_review();

	let post = post.approve();

	assert_eq!("I ate a salad for lunch yesterday", post.content());
}