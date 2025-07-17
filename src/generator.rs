const READER_HTML: &str = include_str!("html/reader.html"); // Default HTML template
const END: &str = "index.html"; // Where to link to once reached end of comic

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use natord::compare;


pub fn generate_reader(path: &str, template_path: &Option<String>) -> std::io::Result<()> {
	let mut entries: Vec<_> = fs::read_dir(path)?
	.filter_map(Result::ok)
	.filter(|entry| {
        	let path = entry.path();
        	path.is_file() && is_image_file(&path)
	})
	.collect();

	// Natural sort order
	entries.sort_by(|a, b| {
		let a_name = a.file_name().to_string_lossy().to_string();
		let b_name = b.file_name().to_string_lossy().to_string();
		compare(&a_name, &b_name)
	});

	// Select reader HTML template. If None, use default
	let mut template = String::new();
	if let Some(path) = template_path {
		File::open(path)?.read_to_string(&mut template)?;
		println!("Custom reader template loaded.");
	} else {
		template = READER_HTML.to_string();
		println!("Default reader template loaded.");
	}


	let title = if let Some(name) = get_directory_name(path) {
		name.to_string()
	} else {
		String::new()
	};
	
	template = template.replace("$TITLE", &title);
	template = template.replace("$TOTAL_PAGES", &(entries.len()-1).to_string());

	for (index, entry) in entries.iter().enumerate() {
		let image_name = entry.file_name().to_string_lossy().to_string();

		let mut html = template.to_string();
		html = html.replace("$IMAGE", &image_name);
		html = html.replace("$CURRENT_PAGE", &index.to_string());

		if (index == entries.len()-1) {
			html = html.replace("$NEXT", END);
		} else { html = html.replace("$NEXT", &format!("{}.html", index + 1)); }

		if (index == 0) {
			html = html.replace("$PREVIOUS", END);
		} else { html = html.replace("$PREVIOUS", &format!("{}.html", index - 1)); }

		let html_path = Path::new(path).join(format!("{}.html", index));
		let mut file = File::create(html_path)?;
		file.write_all(html.as_bytes())?;
	}
	Ok(())
}



fn is_image_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        matches!(
            ext.to_str().unwrap_or("").to_lowercase().as_str(),
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff"
        )
    } else {
        false
    }
}


fn get_directory_name(path: &str) -> Option<&str> {
    Path::new(path).file_name()?.to_str()
}
