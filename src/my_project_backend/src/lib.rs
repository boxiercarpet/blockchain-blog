use crate::blog::Blog;
use std::cell::RefCell;
mod blog;

thread_local! {
    static BLOGS: RefCell<Vec<Blog>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn add_blog(title: String, date: u32, content: String, tags: Vec<String>) -> () {
    let blog = Blog::new(title, date, content, tags);
    BLOGS.with(|blogs| {
        blogs.borrow_mut().push(blog);
    });
}

#[ic_cdk::query]
fn get_blogs() -> Vec<Blog> {
    BLOGS.with(|blogs| blogs.borrow().clone())
}
