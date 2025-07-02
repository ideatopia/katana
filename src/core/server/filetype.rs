#[derive(Debug, Clone)]
pub struct FileType {
    pub extension: String,
    pub content_type: String,
}

impl FileType {
    pub fn new(extension: &str, content_type: &str) -> Self {
        FileType {
            extension: extension.to_string(),
            content_type: content_type.to_string(),
        }
    }

    pub fn all_file_types() -> Vec<Self> {
        vec![
            FileType::new("html", "text/html"),
            FileType::new("css", "text/css"),
            FileType::new("js", "application/javascript"),
            FileType::new("map", "application/json"),
            FileType::new("json", "application/json"),
            FileType::new("xml", "application/xml"),
            FileType::new("txt", "text/plain"),
            FileType::new("md", "text/markdown"),
            FileType::new("jpg", "image/jpeg"),
            FileType::new("jpeg", "image/jpeg"),
            FileType::new("png", "image/png"),
            FileType::new("gif", "image/gif"),
            FileType::new("svg", "image/svg+xml"),
            FileType::new("weba", "audio/webm"),
            FileType::new("webp", "image/webm"),
            FileType::new("webm", "video/webm"),
            FileType::new("ico", "image/x-icon"),
            FileType::new("woff", "font/woff"),
            FileType::new("woff2", "font/woff2"),
            FileType::new("ttf", "font/ttf"),
            FileType::new("otf", "font/otf"),
            FileType::new("mp4", "video/mp4"),
            FileType::new("avi", "video/x-msvideo"),
            FileType::new("mp3", "audio/mpeg"),
            FileType::new("ogg", "audio/ogg"),
            FileType::new("wav", "audio/wav"),
            FileType::new("flac", "audio/flac"),
            FileType::new("zip", "application/zip"),
            FileType::new("tar", "application/x-tar"),
            FileType::new("pdf", "application/pdf"),
            FileType::new("exe", "application/octet-stream"),
            FileType::new("dll", "application/octet-stream"),
            FileType::new("iso", "application/x-iso9660-image"),
        ]
    }

    pub fn from_extension(extension: &str) -> Option<Self> {
        Self::all_file_types()
            .into_iter()
            .find(|ft| ft.extension == extension.to_lowercase())
    }

    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    pub fn is_supported(extension: &str) -> bool {
        Self::from_extension(extension).is_some()
    }

    pub fn content_disposition(&self) -> &'static str {
        // why ? see https://stackoverflow.com/a/1395173/13158370
        "inline"
    }
}
