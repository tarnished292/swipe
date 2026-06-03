pub fn get_category(extension: Option<&str>) -> String {
    match extension {
        Some(
            "rs" | "go" | "py" | "js" | "ts" | "cpp" | "c" | "java" | "html" | "css",
        ) => "programming",
        Some("json" | "toml" | "yaml" | "yml" | "xml" | "config" | "ini") => "configs",
        Some("sh" | "bat" | "ps1") => "scripts",
        Some("msi" | "exe" | "nsis" | "deb" | "rpm" | "dmg" | "iso") | Some("iss") => "installers",
        Some("dll" | "sys" | "tmp") => "system_files",
        Some("zip" | "rar" | "7z" | "tar" | "gz") => "archives",
        Some("pdf") => "pdf_documents",
        Some("doc" | "docx" | "rtf" | "odt") => "word_docs",
        Some("xls" | "xlsx" | "csv") => "spreadsheets",
        Some("ppt" | "pptx") => "presentations",
        Some("txt") => "plain_text",
        Some("png" | "jpg" | "jpeg" | "webp" | "svg" | "ico" | "gif") => "images",
        Some("mp4" | "mkv" | "mov" | "avi") => "videos",
        Some("mp3" | "wav" | "flac" | "aac") => "audios",
        Some(other) => other,
        None => "unknown",
    }
    .to_string()
}