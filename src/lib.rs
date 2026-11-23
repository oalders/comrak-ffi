use std::ffi::{c_char, c_int, CStr, CString};

use comrak::{markdown_to_html, Options};

/// Render GFM markdown to HTML.
///
/// Extensions enabled: tables, strikethrough, autolinks, header IDs.
///
/// # Arguments
/// * `markdown` - A null-terminated C string containing the markdown input.
/// * `unsafe_html` - If non-zero, raw HTML in the markdown is passed through.
///                   If zero, raw HTML is omitted from the output.
///
/// # Returns
/// A newly allocated null-terminated C string containing the rendered HTML.
/// The caller must free this string by passing it to `comrak_free_string`.
#[no_mangle]
pub extern "C" fn comrak_markdown_to_html(
    markdown: *const c_char,
    unsafe_html: c_int,
) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(markdown) };
    let markdown = c_str.to_str().unwrap_or("");

    let mut options = Options::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.autolink = true;
    options.extension.header_id_prefix = Some(String::new());
    options.render.r#unsafe = unsafe_html != 0;

    let html = markdown_to_html(markdown, &options);
    CString::new(html).unwrap().into_raw()
}

/// Free a string previously returned by `comrak_markdown_to_html`.
#[no_mangle]
pub extern "C" fn comrak_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            drop(CString::from_raw(s));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(markdown: &str, unsafe_html: c_int) -> String {
        let input = CString::new(markdown).unwrap();
        let ptr = comrak_markdown_to_html(input.as_ptr(), unsafe_html);
        let html = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_owned();
        comrak_free_string(ptr);
        html
    }

    #[test]
    fn gfm_extensions_enabled() {
        let html = render(
            "# README\n\n| a |\n|---|\n| b |\n\n~~gone~~ https://example.com\n",
            0,
        );
        for expected in [
            "<table>",
            "<del>",
            "<a href=\"https://example.com\">",
            "<h1 id=\"readme\">",
            "href=\"#readme\"",
        ] {
            assert!(html.contains(expected), "missing {expected:?} in {html}");
        }
    }

    #[test]
    fn unsafe_html_controls_raw_html() {
        let markdown = "<script>alert(1)</script>\n";
        let safe = render(markdown, 0);
        assert!(!safe.contains("<script>"), "{safe}");
        let raw = render(markdown, 1);
        assert!(raw.contains("<script>alert(1)</script>"), "{raw}");
    }
}
