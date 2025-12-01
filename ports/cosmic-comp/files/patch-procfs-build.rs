--- cargo-crates/procfs-0.18.0/build.rs.orig	2025-11-26 10:40:03.322706000 -0300
+++ cargo-crates/procfs-0.18.0/build.rs	2025-11-26 10:42:33.170953000 -0300
@@ -1,9 +1,13 @@
 fn main() {
-    // Filters are extracted from `libc` filters
-    let target_os = std::env::var("CARGO_CFG_TARGET_OS").expect("Missing CARGO_CFG_TARGET_OS envvar");
-    if !["android", "linux", "l4re"].contains(&target_os.as_str()) {
-        eprintln!("Building {} on an for a unsupported platform. Currently only linux and android are supported", env!("CARGO_PKG_NAME"));
-        eprintln!("(Your current target_os is {})", target_os);
-        std::process::exit(1)
-    }
+	// Filters are extracted from `libc` filters
+	let target_os =
+		std::env::var("CARGO_CFG_TARGET_OS").expect("Missing CARGO_CFG_TARGET_OS envvar");
+	if !["android", "linux", "l4re", "freebsd"].contains(&target_os.as_str()) {
+		eprintln!(
+			"Building {} on an for a unsupported platform. Currently only linux and android are supported",
+			env!("CARGO_PKG_NAME")
+		);
+		eprintln!("(Your current target_os is {})", target_os);
+		std::process::exit(1)
+	}
 }
