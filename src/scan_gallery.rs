use unicode_segmentation::UnicodeSegmentation;
#[derive(PartialEq)]
#[derive(Debug)]
pub enum GalleryTokenType {
    Split,
    Newline,
    Other,
}
#[derive(PartialEq)]
#[derive(Debug)]
pub struct GalleryToken {
    pub token_type: GalleryTokenType,
    pub value: String,
}
pub fn scan(gallery: &str) -> Vec<GalleryToken> {
    let mut tokens: Vec<GalleryToken> = vec![];
    tokens.push(GalleryToken { 
                token_type: GalleryTokenType::Newline,
                 value: "\r\n".to_string(), });
    let mut graphemes: Vec<&str> = gallery.graphemes(true).collect();
    let mut i = 0;
    while i < graphemes.iter().count() {
        if graphemes[i] == "|" {
            tokens.push(GalleryToken { 
                token_type: GalleryTokenType::Split,
                 value: "|".to_string(), });
        }
        else if graphemes[i] == "\r\n" {
            tokens.push(GalleryToken { 
                token_type: GalleryTokenType::Newline,
                 value: "\r\n".to_string(), });
        }
        else {
            tokens.push(GalleryToken { 
                token_type: GalleryTokenType::Other,
                value: graphemes[i].to_string(),
             });
        }

        i+=1;
    }
    tokens.push(GalleryToken { 
                token_type: GalleryTokenType::Newline,
                 value: "\r\n".to_string(), });
    return tokens;
}