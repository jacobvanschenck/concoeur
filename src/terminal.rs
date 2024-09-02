use libc::{tcgetattr, tcsetattr, termios, ECHO, ICANON, TCSANOW, VMIN, VTIME};
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;

pub fn enter_raw_mode() -> impl FnOnce() {
    // Open stdin for reading
    let stdin = io::stdin().lock();
    // Get the file descriptor for stdin
    let fd = stdin.as_raw_fd();

    // Create a termios structure to hold the terminal attributes
    let mut termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_cc: [0; libc::NCCS],
        c_ispeed: 0,
        c_ospeed: 0,
    };

    // Get the current terminal attributes
    unsafe {
        tcgetattr(fd, &mut termios);
    }

    // Save the original terminal attributes to restore later
    let original_termios = termios;

    // Modify the terminal attributes to set raw mode
    termios.c_lflag &= !(ICANON | ECHO); // Disable canonical mode and echo
    termios.c_cc[VMIN] = 1; // Minimum number of characters for read
    termios.c_cc[VTIME] = 0; // Timeout for read (0 = no timeout)

    // Apply the new terminal attributes
    unsafe {
        tcsetattr(fd, TCSANOW, &termios);
    }

    let restore_fn = move || {
        // Restore the original terminal attributes
        unsafe {
            tcsetattr(fd, TCSANOW, &original_termios);
        }
    };

    restore_fn
}

pub fn clear_screen() {
    let mut stdout = io::stdout();
    write!(stdout, "\x1B[2J\x1B[H").unwrap();
    stdout.flush().unwrap();
}
