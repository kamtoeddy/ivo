use std::{array, collections::HashMap, sync::LazyLock};

use ivo::ivo_schema;
use jiff::Timestamp;

pub use comment_schema::{Comment, CommentId, CommentModel, PartialCommentInput};
pub use post_schema::{PartialPostInput, Post, PostId, PostModel};

#[derive(Clone)]
pub struct PostCtxOptions;

impl PostCtxOptions {
    pub fn new() -> Self {
        Self
    }

    pub async fn find_user_by_title(&self, title: &String) -> Option<&Post> {
        POSTS_BY_TITLE.get(title).cloned()
    }
}

#[ivo_schema(
    input(PostInput, derive(Debug, Clone, PartialEq)),
    output(Post, derive(Debug, Clone, PartialEq)),
    ctx_options(PostCtxOptions)
)]
mod post_schema {
    use super::PostCtxOptions;
    use jiff::Timestamp;

    pub type PostId = i32;

    const MIN_CONTENT_LENGTH: usize = 100;
    const MAX_CONTENT_LENGTH: usize = 2_000;

    const MIN_TITLE_LENGTH: usize = 5;
    const MAX_TITLE_LENGTH: usize = 256;

    struct Fields {
        #[constant(1234)]
        pub id: PostId,

        #[created_at]
        pub created_at: Timestamp,

        #[updated_at]
        pub updated_at: Timestamp,

        #[required]
        #[required_error(|_, _| "\"title\" was not provided!".to_string())]
        #[validate(|v, _, _| {
            let validated = v.trim();

            if validated.len() < MIN_TITLE_LENGTH {
                return Err((
                    format!("\"title\" must be at least {MIN_TITLE_LENGTH} characters long"),
                    None,
                ));
            }

            if validated.len() > MAX_TITLE_LENGTH {
                return Err((
                    format!("\"title\" must be at most {MAX_TITLE_LENGTH} characters long"),
                    None,
                ));
            }

            Ok(Some(validated.to_string()))
        })]
        #[re_validate(async |title, _, o| {
            if o.read().await.find_user_by_title(&title).await.is_some() {
                return Err(("title: \"{title}\" is already taken".into(), None));
            }
            Ok(None)
        })]
        pub title: String,

        #[required]
        #[required_error(|_, _| "\"content\" was not provided!".to_string())]
        #[validate(|v, _, _| {
            let validated = v.trim();

            if validated.len() < MIN_CONTENT_LENGTH {
                return Err((
                    format!("\"content\" must be at least {MIN_CONTENT_LENGTH} characters long"),
                    None,
                ));
            }

            if validated.len() > MAX_CONTENT_LENGTH {
                return Err((
                    format!("\"content\" must be at most {MAX_CONTENT_LENGTH} characters long"),
                    None,
                ));
            }

            Ok(Some(validated.to_string()))
        })]
        pub content: String,
    }

    #[timestamps(Timestamp::now)]
    const _: () = ();

    #[on_success(["content", "title"], |_, _| {
        println!("[options.on_success]: content or title changed");
    })]
    const _: () = ();
}

#[derive(Clone)]
pub struct CommentCtxOptions {
    pub post: Option<Post>,
}

impl CommentCtxOptions {
    pub fn new() -> Self {
        Self { post: None }
    }

    pub async fn get_comment_by_id(&self, id: &CommentId) -> Option<Comment> {
        COMMENTS_BY_ID.get(id).cloned()
    }

    pub async fn get_post_by_id(&self, id: &PostId) -> Option<&Post> {
        POSTS_BY_ID.get(id).cloned()
    }

    pub fn update_post(&mut self, post: Post) {
        self.post = Some(post);
    }
}

#[ivo_schema(
    input(CommentInput, derive(Debug, Clone, PartialEq)),
    output(Comment, derive(Debug, Clone, PartialEq)),
    ctx_options(CommentCtxOptions)
)]
mod comment_schema {
    use super::{CommentCtxOptions, PostId};
    use jiff::Timestamp;

    pub type CommentId = i32;

    const MIN_CONTENT_LENGTH: usize = 1;
    const MAX_CONTENT_LENGTH: usize = 1_000;
    const COMMENT_NOT_FOUND_ERROR: &str = "Comment not found";
    const POST_NOT_FOUND_ERROR: &str = "Post not found";

    struct Fields {
        #[constant(1234)]
        pub id: CommentId,

        #[created_at]
        pub created_at: Timestamp,

        #[updated_at]
        pub updated_at: Timestamp,

        #[required]
        #[required_error(|_, _| "\"content\" was not provided!".to_string())]
        #[validate(|v, _, _| {
            let validated = v.trim();

            if validated.len() < MIN_CONTENT_LENGTH {
                return Err((
                    format!("\"content\" must be at least {MIN_CONTENT_LENGTH} characters long"),
                    None,
                ));
            }

            if validated.len() > MAX_CONTENT_LENGTH {
                return Err((
                    format!("\"content\" must be at most {MAX_CONTENT_LENGTH} characters long"),
                    None,
                ));
            }

            Ok(Some(validated.to_string()))
        })]
        pub content: String,

        #[readonly]
        #[required]
        #[required_error(|_, _| "\"post\" is required!".to_string())]
        #[re_validate(async |id, _, o| {
            let mut guard = o.write().await;

            let Some(post) = guard.get_post_by_id(&id).await.cloned() else {
                return Err((
                    POST_NOT_FOUND_ERROR.to_string(),
                    None,
                ));
            };

            guard.update_post(post);

            Ok(None)
        })]
        pub post: PostId,

        #[readonly]
        #[lax(None)]
        #[re_validate(async |id, ctx, o| {
            if id.is_none() {
                return Ok(None);
            }

            let post_id = ctx.values().post;

            let guard = o.read().await;

            if let Some(c) = guard.get_comment_by_id(&id.unwrap()).await {
                // cannot reply to a comment that does not belong to the same post
                if c.post != post_id {
                    return Err((
                        COMMENT_NOT_FOUND_ERROR.to_string(),
                        None,
                    ));
                }
            } else {
                return Err((
                    COMMENT_NOT_FOUND_ERROR.to_string(),
                    None,
                ));
            }

            Ok(None)
        })]
        pub reply_to: Option<CommentId>,
    }

    #[timestamps(Timestamp::now)]
    const _: () = ();
}

static POSTS_LIST: LazyLock<[Post; 3]> = LazyLock::new(|| {
    array::from_fn(|i| {
        let id = (i as i32) + 1;
        let title = format!("post-{id}");

        let now = Timestamp::now();

        Post {
            id,
            created_at: now,
            updated_at: now,
            content: format!("post-{id}-content"),
            title,
        }
    })
});

static POSTS_BY_ID: LazyLock<HashMap<&PostId, &Post>> = LazyLock::new(|| {
    let collection: [(&PostId, &Post); 3] = array::from_fn(|i| {
        let p = POSTS_LIST.get(i).unwrap();
        (&p.id, p)
    });

    HashMap::from(collection)
});

static POSTS_BY_TITLE: LazyLock<HashMap<&String, &Post>> = LazyLock::new(|| {
    let collection: [(&String, &Post); 3] = array::from_fn(|i| {
        let p = POSTS_LIST.get(i).unwrap();
        (&p.title, p)
    });

    HashMap::from(collection)
});

static COMMENTS_BY_ID: LazyLock<HashMap<CommentId, Comment>> = LazyLock::new(|| {
    let collection: [(CommentId, Comment); 3] = array::from_fn(|i| {
        let id = (i as i32) + 1;
        let content = format!("comment-{id}");

        let now = Timestamp::now();

        (
            id,
            Comment {
                id,
                created_at: now,
                updated_at: now,
                content,
                post: id,
                reply_to: None,
            },
        )
    });

    HashMap::from(collection)
});
