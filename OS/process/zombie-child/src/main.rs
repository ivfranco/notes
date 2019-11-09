use nix::unistd::{fork, sleep};

fn main() -> nix::Result<()> {
    if fork()?.is_parent() {
        sleep(1_000_000);
    }

    Ok(())
}
