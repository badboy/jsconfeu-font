use std::time::Duration;
use std::thread::sleep;
use std::env;

mod font;
use font::FONT;

const ROWS : usize = 20;
const COLS : usize = 36;
const BUFLEN : usize = ROWS * COLS;

fn pos(x: usize, y: usize) -> usize {
    y * COLS + x
}

fn render(buf: &[u8]) {
    assert!(buf.len() == BUFLEN);

    for y in 0..ROWS {
        for x in 0..COLS {
            match buf[pos(x,y)] {
                1 => print!("*"),
                _ => print!(" "),
            }
        }
        println!();
    }
}

fn add_char(posx: usize, posy: usize, c: u8, buf: &mut [u8]) {
    let mut char_data = FONT[c as usize];

    for y in (posy..(posy+8)).rev() {
        for x in (posx..(posx+8)).rev() {
            let bit = char_data & 0x1;
            char_data = char_data >> 1;
            if bit == 1 {
                buf[pos(x, y)] = 1;
            } else {
                buf[pos(x, y)] = 0;
            }
        }
    }
}

fn add_char_trimmed(posx: isize, posy: usize, c: u8, buf: &mut [u8]) {
    let mut char_data = FONT[c as usize];

    for y in (posy..(posy+8)).rev() {
        for x in (posx..(posx+8)).rev() {
            let xu = x as usize;
            let bit = char_data & 0x1;
            char_data = char_data >> 1;

            // trim it outside of the viewport
            if x < 0 || xu >= COLS || y >= ROWS { continue; }

            if bit == 1 {
                buf[pos(xu, y)] = 1;
            } else {
                buf[pos(xu, y)] = 0;
            }
        }
    }
}

fn add_string(mut posx: usize, mut posy: usize, msg: &str, buf: &mut [u8]) {
    if !msg.is_ascii() { return; } // no non-ascii support

    for c in msg.bytes() {
        add_char(posx, posy, c, buf);
        posx += 8;
        if (posx+8) > COLS {
            posx = 0;
            posy += 8;
        }
    }
}

fn add_string_trimmed(mut posx: isize, mut posy: usize, msg: &str, buf: &mut [u8]) {
    if !msg.is_ascii() { return; } // no non-ascii support

    for c in msg.bytes() {
        add_char_trimmed(posx, posy, c, buf);
        posx += 8;
        if posx > 0 && posx as usize >= COLS { return; }
    }
}

fn main() {
    let mut buffer = vec![0; BUFLEN];
    let msg = env::args().skip(1).next();

    if let Some(msg) = msg {
        for x in 0..(msg.len()*8 + msg.len()*8) {
            add_string_trimmed(COLS as isize - x as isize, 0, &msg, &mut buffer);
            render(&buffer);
            for _ in 0..COLS {
               print!("-");
            }
            println!("-");
            sleep(Duration::from_millis(100));
        }
    } else {
        add_string(0, 0, "abc", &mut buffer);
        render(&buffer);
    }
}
