<div align="center">
  <img src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQxXB9P5fDpjzPfOGEjLErH2cYlxkTf0oBxCw&s" width="100" />
  <h1>Swipe</h1>
  <p>A command-line file organizer written in Rust</p>
</div>

---

## What is Swipe

Swipe is a CLI tool that takes a folder path as input and organizes its contents by automatically sorting files into categorized subfolders based on their extensions. Images go into images, videos go into videos, documents go into documents, and so on. It removes the manual effort of cleaning up cluttered directories.

## Usage

## Arguments
| Argument | Description | Required |
|---|---|---|
| `<input_path>` | Folder to organize | ✅ Yes |
| `[output_path]` | Where to put organized files | ❌ No (default: `<input_path>/swipe_output`) |

## Examples

**Full path:**
```bash
swipe C:/Users/You/Downloads
swipe C:/Users/You/Downloads C:/Users/You/Desktop/cleaned
```

**Already in the folder (use `.` for current directory):**
```bash
cd C:/Users/You/Downloads
swipe .
swipe . C:/Users/You/Desktop/cleaned
```

**Relative path:**
```bash
swipe ./my-folder
swipe ./my-folder ./cleaned
```

Swipe will read the folder, detect each file's extension, create the appropriate category folder if it does not exist, and move the file into it.

## Categories

| Category | Extensions |
|----------|------------|
| images | png, jpg, jpeg, gif, bmp, webp, svg, ico, tiff, raw, heic |
| videos | mp4, mkv, avi, mov, wmv, flv, webm, m4v |
| audio | mp3, wav, flac, aac, ogg, m4a, wma, opus |
| documents | pdf, txt, csv, docx, doc, xlsx, xls, pptx, ppt, odt, rtf |
| sheet | json, xml, yaml, yml, toml, ini, env, conf |
| code | rs, py, js, ts, go, c, cpp, h, java, kt, swift, rb, php, cs, lua |
| web | html, css, scss, jsx, tsx, vue, svelte |
| build | lock, dockerfile, makefile, gradle, cmake |
| installer | exe, msi, dmg, deb, rpm, apk, appimage |
| archives | zip, tar, gz, rar, 7z, bz2, xz |
| fonts | ttf, otf, woff, woff2, eot |
| database | db, sqlite, sql, mdb |
Files with unrecognized extensions are sorted into a folder named after their extension. Files with no extension go into folder/unknown.

## Project Philosophy

The goal of Swipe is to be a single-purpose tool that does one thing well. No configuration files, no dependencies, no installation beyond the binary. You point it at a folder and it cleans it. The code is kept minimal and readable so that someone learning Rust can follow along with the logic without getting lost.

## Why Rust

Rust was chosen for this project because it handles file system operations safely without a garbage collector. When you move hundreds of files, you want predictable performance and no hidden runtime surprises. Rust's ownership model means every file path and string is accounted for at compile time, and errors from the file system are forced to be handled explicitly rather than silently failing. The result is a tool that is fast, reliable, and honest about what went wrong when something does.

Beyond the technical reasons, Rust teaches you to think carefully about your data. Writing even a simple tool like this in Rust builds habits that make you a better programmer in any language.

## License

MIT
