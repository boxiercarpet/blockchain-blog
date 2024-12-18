use crate::blog::Blog;
use std::cell::RefCell;
mod blog;

thread_local! {
    static BLOGS: RefCell<Vec<Blog>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn add_blog(title: String, content: String, tags: Vec<String>) -> Result<String, String> {
    if title.len() > 250 {
        return Err("Title is too long".to_string());
    }

    let blog = Blog::new(title, content, tags);
    BLOGS.with(|blogs| {
        blogs.borrow_mut().push(blog);
    });

    Ok("Blog added".to_string())
}

#[ic_cdk::query]
fn get_blogs() -> Vec<Blog> {
    BLOGS.with(|blogs| blogs.borrow().clone())
}
