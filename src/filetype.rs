#[derive(Debug)]
#[derive(Clone)]
pub struct FileType {
    pub extension: String,
    pub content_type: String,
    pub name: String,
}

impl FileType {
    pub fn new(extension: &str, content_type: &str, name: &str) -> Self {
        FileType {
            extension: extension.to_string(),
            content_type: content_type.to_string(),
            name: name.to_string(),
        }
    }

    pub fn all_file_types() -> Vec<Self> {
        vec![
            FileType::new("html", "text/html", "HTML"),
            FileType::new("css", "text/css", "CSS"),
            FileType::new("js", "application/javascript", "JavaScript"),
            FileType::new("map", "application/json", "JavaScript Source Map"),
            FileType::new("json", "application/json", "JSON"),
            FileType::new("xml", "application/xml", "XML"),
            FileType::new("txt", "text/plain", "Text File"),
            FileType::new("md", "text/markdown", "Markdown"),
            FileType::new("jpg", "image/jpeg", "JPEG Image"),
            FileType::new("jpeg", "image/jpeg", "JPEG Image"),
            FileType::new("png", "image/png", "PNG Image"),
            FileType::new("gif", "image/gif", "GIF Image"),
            FileType::new("svg", "image/svg+xml", "SVG Image"),
            FileType::new("weba", "audio/webm", "WebA Audio"),
            FileType::new("webp", "image/webm", "WebP Image"),
            FileType::new("webm", "video/webm", "WebM Video"),
            FileType::new("ico", "image/x-icon", "ICO Image"),
            FileType::new("woff", "font/woff", "Web Open Font Format"),
            FileType::new("woff2", "font/woff2", "Web Open Font Format 2"),
            FileType::new("ttf", "font/ttf", "TrueType Font"),
            FileType::new("otf", "font/otf", "OpenType Font"),
            FileType::new("mp4", "video/mp4", "MP4 Video"),
            FileType::new("avi", "video/x-msvideo", "AVI Video"),
            FileType::new("mp3", "audio/mpeg", "MP3 Audio"),
            FileType::new("ogg", "audio/ogg", "OGG Audio"),
            FileType::new("wav", "audio/wav", "WAV Audio"),
            FileType::new("flac", "audio/flac", "FLAC Audio"),
            FileType::new("zip", "application/zip", "ZIP Archive"),
            FileType::new("tar", "application/x-tar", "TAR Archive"),
            FileType::new("pdf", "application/pdf", "PDF Document"),
            FileType::new("exe", "application/octet-stream", "Executable File"),
            FileType::new("dll", "application/octet-stream", "Dynamic Link Library"),
            FileType::new("iso", "application/x-iso9660-image", "ISO Disk Image"),
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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_supported(extension: &str) -> bool {
        Self::from_extension(extension).is_some()
    }

    pub fn is_binary_extension(extension: &str) -> bool {
        matches!(extension.to_lowercase().as_str(),
            "jpeg" | "jpg" | "png" | "gif" | "svg" | "webp" | "ico" |
            "mp4" | "webm" | "avi" | "mp3" | "ogg" | "wav" | "flac" |
            "zip" | "tar" | "pdf" | "exe" | "dll" | "iso"
        )
    }

    pub fn content_disposition(&self) -> &'static str {
        match self.extension.to_lowercase().as_str() {
            "html" | "htm" | "txt" | "css" | "js" | "js.map" |
            "json" | "xml" | "svg" | "pdf" | "jpeg" | "jpg" |
            "png" | "gif" | "webp" | "ico"
            => "inline",
            _ => "attachment",
        }
    }
}
