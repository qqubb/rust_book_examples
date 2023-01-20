
// Listing 17-12: Definition of a Post struct and a new function that creates a new Post instance, a State trait, and a Draft struct

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    
    // Listing 17-13: Implementing the add_text method to add text to a post’s content
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    
    // Listing 17-14: Adding a placeholder implementation for the content method on Post that always returns an empty string slice
    pub fn content(&self) -> &str {
        // Listing 17-17: Updating the content method on Post to delegate to a content method on State
        self.state.as_ref().unwrap().content(self)
    }
    
    // Listing 17-15: Implementing request_review methods on Post and the State trait
    pub fn request_review(&mut self) {
    
        // To consume the old state, the request_review method needs to take ownership of the state value. 
        // This is where the Option in the state field of Post comes in:
        // we call the take method to take the Some value out of the state field and 
        // leave a None in its place, because Rust doesn’t let us have unpopulated fields in structs. 
        // This lets us move the state value out of Post rather than borrowing it. 
        // Then we’ll set the post’s state value to the result of this operation.
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }         
    
    // Listing 17-16: Implementing the approve method on Post and the State trait
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }    
    
}

// Listing 17-15: Implementing request_review methods on Post and the State trait
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    // Listing 17-16: Implementing the approve method on Post and the State trait
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // Listing 17-18: Adding the content method to the State trait
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }    
    
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    
    // Listing 17-16: Implementing the approve method on Post and the State trait
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }    
    
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    // Listing 17-16: Implementing the approve method on Post and the State trait
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }    
}

// Listing 17-16: Implementing the approve method on Post and the State trait
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    // Listing 17-18: Adding the content method to the State trait
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

/*
// Listing 17-19: A Post with a content method and a DraftPost without a content method
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

impl DraftPost {
    // --snip--
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

// Listing 17-20: A PendingReviewPost that gets created by calling request_review on DraftPost and an approve method that turns a PendingReviewPost into a published Post
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
*/
