pub struct Post {
    content: String,
}

pub struct Draft {
    content: String,
}

impl Post {
    pub fn new() -> Draft {
        Draft {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl Draft {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approval_count: 0,
        }
    }
}


pub struct PendingReviewPost {
    content: String,
    approval_count: u8,
}

impl PendingReviewPost {
    pub fn approve(&mut self) -> Option<Post> {
        // TODO: dit is ontzettend vies. Ipv pure type transformatie state toevoegen. Ideeën voor nettere opties:
        //  - nieuwe struct OnceReviewedPost
        //  - approve() een boolean laten teruggeven; true als post klaar is voor publicatie
        if self.approval_count < 1 {
            self.approval_count += 1;
            None
        } else {
            Some(Post {
                content: self.content.clone() // OF: String::from(*self.content)
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blog_lifecycle() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        // assert_eq!("", post.content());  // should not compile - a Draft can't produce content

        let mut post = post.request_review();
        // assert_eq!("", post.content());  // non compila - no content() function on PendingReviewPost struct

        let first_approval = post.approve();
        assert!(first_approval.is_none());

        let post = post.approve();
        assert_eq!("I ate a salad for lunch today", post.unwrap().content());
    }

    #[test]
    fn test_private_constructor() {
        let another = Post { content : String::from("dit kan alleen in deze module") };
        assert_eq!("dit kan alleen in deze module", another.content);
    }

}