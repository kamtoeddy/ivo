use std::time::Instant;

mod domain;

use crate::domain::{
    CommentCtxOptions, CommentModel, PartialCommentInput, PartialPostInput, PostCtxOptions,
    PostModel,
};

#[async_std::main]
async fn main() {
    // required error (tile, content)
    // let input = PartialPostInput::new();

    // validation error (tile already taken)
    // let input = PartialPostInput::new()
    //     .with_title("post-1".into())
    //     .with_content("some content".repeat(10));

    // success
    let input = PartialPostInput::new()
        .with_title("a blog post".into())
        .with_content("some content ".repeat(10));

    let timer = Instant::now();

    let r = PostModel.create(input, PostCtxOptions::new()).await;

    println!("\nCreate duration: {:?}", timer.elapsed());

    match r {
        Ok((data, _, handle_success)) => {
            println!("\n{:#?}\n", data);

            handle_success();
        }
        Err((errors, _)) => {
            println!("\nFailed to create: {:#?}", errors);
        }
    };

    // required error (content, post)
    // let input = PartialCommentInput::new();

    // validation error (post, reply_to)
    // let input = PartialCommentInput::new()
    //     .with_post(20)
    //     .with_reply_to(Some(11))
    //     .with_content("some content ".repeat(10));

    // success
    // let input = PartialCommentInput::new()
    //     .with_post(1)
    //     .with_content("some content ".repeat(10));

    // success
    let input = PartialCommentInput::new()
        .with_post(2)
        .with_reply_to(Some(3))
        .with_content("some content ".repeat(10));

    let timer = Instant::now();

    let r = CommentModel.create(input, CommentCtxOptions::new()).await;

    println!("\nCreate duration: {:?}", timer.elapsed());

    match r {
        Ok((data, _)) => {
            println!("\n{:#?}\n", data);
        }
        Err((errors, _)) => {
            println!("\nFailed to create: {:#?}", errors);
        }
    };
}
