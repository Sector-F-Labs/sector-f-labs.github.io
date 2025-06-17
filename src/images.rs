use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Extracts image paths from HTML content.
///
/// This function uses regex to find all <img> tags in the HTML and extracts
/// their src attribute values.
///
/// # Arguments
/// * `html` - The HTML content to scan for images
///
/// # Returns
/// * `Vec<String>` - A vector of image paths found in the HTML
///
/// # Examples
/// ```
/// let html = r#"<img src="./image1.png" alt="test"> <img src="photos/image2.jpg">"#;
/// let paths = extract_image_paths(html);
/// assert_eq!(paths, vec!["./image1.png", "photos/image2.jpg"]);
/// ```
pub fn extract_image_paths(html: &str) -> Vec<String> {
    let img_regex = Regex::new(r#"<img[^>]*src=["']([^"'>]+)["'][^>]*>"#).unwrap();
    img_regex
        .captures_iter(html)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect()
}

/// Copies images from source directory to output directory with flattened structure.
///
/// This function takes a list of image paths, finds them in the source directory,
/// and copies them to the output directory using only the filename (flattening
/// any directory structure).
///
/// # Arguments
/// * `source_dir` - The source directory where images are located
/// * `output_dir` - The destination directory where images should be copied
/// * `image_paths` - A slice of image paths to copy
///
/// # Returns
/// * `Result<Vec<String>>` - A vector of copied image filenames on success, or an error
///
/// # Examples
/// ```
/// let paths = vec!["images/photo.jpg".to_string(), "assets/logo.png".to_string()];
/// let copied = copy_images_flat("./src", "./dist", &paths)?;
/// // copied will contain ["photo.jpg", "logo.png"]
/// ```
pub fn copy_images_flat(
    source_dir: &str,
    output_dir: &str,
    image_paths: &[String],
) -> Result<Vec<String>> {
    let mut copied_images = Vec::new();

    for img_path in image_paths {
        let img_source = Path::new(source_dir).join(img_path);

        if img_source.exists() {
            // Only use the filename, not the path
            let filename = Path::new(img_path)
                .file_name()
                .ok_or_else(|| format!("Invalid filename in path: {}", img_path))?;

            let dest_path = Path::new(output_dir).join(filename);

            // Create output directory if it doesn't exist
            fs::create_dir_all(output_dir)?;

            // Copy the image file
            fs::copy(&img_source, &dest_path)?;

            copied_images.push(filename.to_string_lossy().to_string());
        }
    }

    Ok(copied_images)
}

/// Fixes image paths in HTML to use flattened filenames.
///
/// This function updates all image src attributes in the HTML to point to
/// flattened filenames (just the filename without directory structure).
/// It replaces the original paths with "./filename" format.
///
/// # Arguments
/// * `html` - The HTML content to process
/// * `image_paths` - A slice of original image paths that were flattened
///
/// # Returns
/// * `String` - The HTML with updated image paths
///
/// # Examples
/// ```
/// let html = r#"<img src="assets/images/photo.jpg" alt="photo">"#;
/// let paths = vec!["assets/images/photo.jpg".to_string()];
/// let fixed = fix_image_paths(html, &paths);
/// // Result: <img src="./photo.jpg" alt="photo">
/// ```
pub fn fix_image_paths(html: &str, image_paths: &[String]) -> String {
    let mut result = html.to_string();

    for img_path in image_paths {
        let filename = Path::new(img_path).file_name().unwrap().to_string_lossy();

        let pattern = format!(r#"src=["'][^"'>]*{}["']"#, regex::escape(&filename));
        let replacement = format!(r#"src="./{}""#, filename);

        result = Regex::new(&pattern)
            .unwrap()
            .replace_all(&result, replacement)
            .to_string();
    }

    result
}

/// Processes images in HTML content: extracts, copies, and fixes paths.
///
/// This is a convenience function that performs the complete image processing workflow:
/// 1. Extracts image paths from HTML
/// 2. Copies images to output directory with flattened structure
/// 3. Updates HTML to use the new flattened paths
///
/// # Arguments
/// * `html` - The HTML content to process
/// * `source_dir` - The source directory where images are located
/// * `output_dir` - The destination directory where images should be copied
///
/// # Returns
/// * `Result<String>` - The processed HTML with updated image paths on success, or an error
pub fn process_images(html: &str, source_dir: &str, output_dir: &str) -> Result<String> {
    let image_paths = extract_image_paths(html);
    let copied_images = copy_images_flat(source_dir, output_dir, &image_paths)?;
    Ok(fix_image_paths(html, &copied_images))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_image_paths() {
        let html = r#"
            <img src="./image1.png" alt="test">
            <img src="photos/image2.jpg" alt="photo">
            <img class="logo" src="../assets/logo.svg">
        "#;

        let paths = extract_image_paths(html);
        assert_eq!(paths.len(), 3);
        assert!(paths.contains(&"./image1.png".to_string()));
        assert!(paths.contains(&"photos/image2.jpg".to_string()));
        assert!(paths.contains(&"../assets/logo.svg".to_string()));
    }

    #[test]
    fn test_fix_image_paths() {
        let html = r#"<img src="assets/images/photo.jpg" alt="photo">"#;
        let paths = vec!["assets/images/photo.jpg".to_string()];
        let result = fix_image_paths(html, &paths);
        assert!(result.contains(r#"src="./photo.jpg""#));
    }

    #[test]
    fn test_extract_image_paths_empty() {
        let html = "<p>No images here</p>";
        let paths = extract_image_paths(html);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_fix_image_paths_multiple() {
        let html = r#"
            <img src="dir1/photo1.jpg" alt="1">
            <img src="dir2/photo2.png" alt="2">
        "#;
        let paths = vec!["dir1/photo1.jpg".to_string(), "dir2/photo2.png".to_string()];
        let result = fix_image_paths(html, &paths);
        assert!(result.contains(r#"src="./photo1.jpg""#));
        assert!(result.contains(r#"src="./photo2.png""#));
    }
}
