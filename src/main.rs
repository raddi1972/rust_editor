use std::collections::VecDeque;
use std::env::args;
use std::fs;
use std::io::{stdin, stdout, Write};
use termion::color::Color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style};

struct Doc {
    lines: Vec<String>,
}

#[derive(Debug)]
struct Coordinates {
    pub x: usize,
    pub y: usize,
}

struct TextViewer {
    doc: Doc,
    buffer: VecDeque<String>,
    doc_length: usize,
    cur_pos: Coordinates,
    terminal_size: Coordinates,
    file_size: String,
    starting_index: i32,
}

impl TextViewer {
    fn init(file_name: &str) -> Self {
        let mut doc_file = Doc { lines: vec![] };
        let file_handle = fs::read_to_string(file_name).expect("Unable to Read the String");

        for doc_line in file_handle.lines() {
            doc_file.lines.push(doc_line.to_string());
        }

        let mut doc_length = file_handle.lines().count();

        let size = termion::terminal_size().expect("Unable to get terminal size");

        let mut window_buffer = VecDeque::new();

        for it in 0..std::cmp::min(doc_file.lines.len(), size.1 as usize - 3) {
            window_buffer.push_back(String::from(&doc_file.lines[it]));
        }

        Self {
            doc: doc_file,
            doc_length: doc_length,
            cur_pos: Coordinates { x: 1, y: 1 },
            buffer: window_buffer,
            terminal_size: Coordinates {
                x: size.0 as usize,
                y: size.1 as usize,
            },
            file_size: file_name.into(),
            starting_index: 0,
        }
    }

    fn show_document(&mut self) {
        let pos = &self.cur_pos;
        let (old_x, old_y) = (pos.x, pos.y);
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        println!(
            "{}{}Welcome to Super text Viewer\r{}",
            color::Bg(color::Black),
            color::Fg(color::White),
            style::Reset
        );

        for line in self.buffer.iter() {
            println!("{}\r", line);
        }

        // if self.doc_length < self.terminal_size.y {
        //     for line in 0..self.doc_length {
        //         println!("{}\r", self.doc.lines[line as usize]);
        //     }
        // } else {
        //     if pos.y <= self.terminal_size.y {
        //         for line in 0..self.terminal_size.y - 3 {
        //             println!("{}\r", self.doc.lines[line as usize]);
        //         }
        //     } else {
        //         for line in pos.y - (self.terminal_size.y - 3)..pos.y {
        //             println!("{}\r", self.doc.lines[line as usize]);
        //         }
        //     }
        // }

        println!(
            "{}",
            termion::cursor::Goto(0, (self.terminal_size.y - 2) as u16),
        );
        print!(
            "{}{} line-count={} Filename: {}{}",
            color::Fg(color::Red),
            style::Bold,
            self.doc_length,
            self.file_size,
            style::Reset
        );
        self.set_pos(old_x, old_y);
    }

    fn set_pos(&mut self, x: usize, y: usize) {
        self.cur_pos.x = x;
        self.cur_pos.y = y;

        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
        );
    }

    fn run(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('q') => {
                    break;
                }
                Key::Left => {
                    self.dec_x();
                    self.show_document();
                }
                Key::Right => {
                    self.inc_x();
                    self.show_document();
                }
                Key::Down => {
                    self.inc_y();
                }
                Key::Up => {
                    self.dec_y();
                }
                Key::Backspace => {
                    self.dec_x(); // TODO - need to check if the show_document is needed here or not
                }
                _ => {
                    self.show_document();
                }
            }

            stdout.flush().unwrap();
        }
    }

    fn inc_x(&mut self) {
        if (self.cur_pos.x < self.terminal_size.x) {
            self.cur_pos.x += 1;
        }
        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
        );
    }

    fn dec_x(&mut self) {
        if (self.cur_pos.x > 1) {
            self.cur_pos.x -= 1;
        }
        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
        );
    }
    fn inc_y(&mut self) {
        let max_window_size = self.terminal_size.y - 3;
        if self.cur_pos.y < max_window_size {
            self.cur_pos.y += 1;
        } else if self.cur_pos.y == max_window_size {
            let final_index = self.starting_index + max_window_size as i32;
            if final_index < self.doc.lines.len() as i32 {
                self.starting_index += 1;
                self.buffer
                    .push_back(String::from(&self.doc.lines[final_index as usize]));
                self.buffer.pop_front();
                self.show_document();
            }
        }
        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
        );
    }
    fn dec_y(&mut self) {
        if (self.cur_pos.y > 1) {
            self.cur_pos.y -= 1;
        } else if self.cur_pos.y == 1 {
            if self.starting_index > 0 {
                self.starting_index -= 1;
                self.buffer.pop_back();
                self.buffer
                    .push_front(String::from(&self.doc.lines[self.starting_index as usize]));
                self.show_document();
            }
        }
        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
        );
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Please provide a file as the argument");
        std::process::exit(0);
    }

    // check if the file exists or not
    if !std::path::Path::new(&args[1]).exists() {
        println!("File does not exists");
        std::process::exit(0);
    }

    // Open file & load into struct
    println!("{}", termion::cursor::Show);
    // Initialize viewer
    let mut viewer = TextViewer::init(&args[1]);
    viewer.show_document();
    viewer.run();
}
