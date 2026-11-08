use crate::gallery::scan_gallery::{GalleryToken, GalleryTokenType};

#[derive(PartialEq)]
#[derive(Debug)]
pub struct GalleryItem {
    pub image: String,
    pub caption: String,
}
pub fn parse(tokens: Vec<GalleryToken>) -> Vec<GalleryItem> {
    let mut gallery_items: Vec<GalleryItem> = vec![];
    let mut i = 0;

    while i < tokens.len() {
        if tokens[i].token_type == GalleryTokenType::Newline {
            let mut image = String::from("");
            i+=1;
            while i < tokens.len() && tokens[i].token_type != GalleryTokenType::Split {
                image += &tokens[i].value;
                i+=1;
            }
            i+=1;
            let mut caption = String::from("");
            while i < tokens.len() && tokens[i].token_type != GalleryTokenType::Newline {
                caption += &tokens[i].value;
                i+=1;
            }
            gallery_items.push(GalleryItem { image, caption });
        }

        i+=1;
    }

    return gallery_items;
}