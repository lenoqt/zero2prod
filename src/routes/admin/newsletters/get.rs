//! src/routes/admin/newsletter/get.rs
use std::fmt::Write;

use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;

pub async fn newsletter_publisher(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            
            <head>
                <meta http-equiv="content-type" content="text/html" ; charset=utf-8">
                <title>Publish newsletters</title>
            
                <head>
            
                <body>
                    {msg_html}
                    <form action="/admin/newsletters" method="POST">
                        <label>Title:<br>
                            <input type="text" placeholder="Enter the issue title" name="title">
                        </label>
                        <br>
                        <label>Text content:<br>
                            <textarea placeholder="Enter the content in plain text" name="text_content" rows="5" cols="30"></textarea>
                        </label>
                        <br>
                        <label>HTML content:<br>
                            <textarea placeholder="Enter the content in HTML format" name="html_content" rows="5" cols="30"></textarea>
                        </label>
                        <br>
                        <button type="submit" name="postnewsletter">
                            Post newsletter
                        </button>
                    </form>
                    <p><a href="/admin/dashboard">&lt;- Back</a></p>
                </body>
            
            </html>
        "#,
        )))
}
