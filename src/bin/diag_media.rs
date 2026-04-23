use media_remote::{NowPlaying, NowPlayingPerl};
use sketchybarrc::media_ffi;

fn main() {
    println!("--- Media Info Diagnostic ---");

    // 1. Custom FFI
    println!("Testing Custom FFI (Primary)...");
    match media_ffi::get_now_playing_info() {
        Some(info) => {
            println!("  ✅ SUCCESS");
            println!("  Artist:   {:?}", info.artist);
            println!("  Title:    {:?}", info.title);
            println!("  Album:    {:?}", info.album);
            println!("  Elapsed:  {:?}", info.elapsed_time);
            println!("  Duration: {:?}", info.duration);
            println!(
                "  Artwork:  {} bytes",
                info.artwork_data.map(|d| d.len()).unwrap_or(0)
            );
        }
        None => println!("  ❌ FFI returned None"),
    }

    // 2. Crate Native (NowPlaying)
    println!("\n2. Testing Crate Native (NowPlaying)...");
    let now_playing = NowPlaying::new();
    std::thread::sleep(std::time::Duration::from_millis(1000)); // Wait for notifications
    match &*now_playing.get_info() {
        Some(info) => {
            println!("  ✅ SUCCESS");
            println!("  Artist:   {:?}", info.artist);
            println!("  Title:    {:?}", info.title);
        }
        None => println!("  ❌ Native returned None"),
    }

    // 3. Crate Perl Fallback
    println!("\n3. Testing Perl Fallback (Secondary)...");
    let perl = NowPlayingPerl::new();
    std::thread::sleep(std::time::Duration::from_millis(1000)); // Wait for process/notifications
    match &*perl.get_info() {
        Some(info) => {
            println!("  ✅ SUCCESS");
            println!("  Artist:   {:?}", info.artist);
            println!("  Title:    {:?}", info.title);
            println!("  Album:    {:?}", info.album);
            println!("  Elapsed:  {:?}", info.elapsed_time);
            println!("  Duration: {:?}", info.duration);
        }
        None => println!("  ❌ Perl Fallback returned None"),
    }
}
