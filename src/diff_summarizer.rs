use regex::Regex;

pub fn summarize(raw_diff: &str) -> String {
    let mut summarized = String::new();

    // Regex to match diff headers for specific files
    let file_header_re = Regex::new(r"(?m)^diff --git a/(.+) b/(.+)").unwrap();

    // Files to ignore completely (lockfiles, generated assets)
    let ignore_patterns = [
        "Cargo.lock",
        "package-lock.json",
        "yarn.lock",
        "pnpm-lock.yaml",
        ".min.js",
        ".min.css",
        "dist/",
        "build/",
    ];

    let mut current_file_ignored = false;

    for line in raw_diff.lines() {
        if let Some(caps) = file_header_re.captures(line) {
            let file_name = caps.get(2).map_or("", |m| m.as_str());
            current_file_ignored = ignore_patterns.iter().any(|p| file_name.contains(p));

            if current_file_ignored {
                summarized.push_str(&format!(
                    "diff --git a/{} b/{} [SKIPPED: Lockfile/Generated]\n\n",
                    file_name, file_name
                ));
            } else {
                summarized.push_str(line);
                summarized.push('\n');
            }
        } else if !current_file_ignored {
            // Truncate extremely long lines (e.g., minified code that slipped through)
            if line.len() > 200 {
                summarized.push_str(&line[..200]);
                summarized.push_str("... [TRUNCATED]\n");
            } else {
                summarized.push_str(line);
                summarized.push('\n');
            }
        }
    }

    summarized
}
