use std::{
    sync::mpsc::{Receiver, TryRecvError, self},
    io::{Stdout, Write},
    thread,
};

use portable_pty::{
    CommandBuilder,
    native_pty_system,
    PtySize, 
    PtySystem,
    PtyPair,
};

use crossterm::event::{Event, KeyCode, KeyModifiers};

use super::element::Element;

pub struct PtyView {
    pty_system: Box<dyn PtySystem>,
    pty_pair: PtyPair,
    rx: Receiver<Vec<u8>>, 
    writer: Box<dyn Write + Send>,
    buffer: [u8; 2048],
}

impl PtyView {
    pub fn new() -> Self {
        // Launch neovim in pty
        let pty_system = native_pty_system();

        let mut pair = pty_system.openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        }).expect("Couldn't open pty");

        let mut cmd = CommandBuilder::new("nano");

        let mut child = pair.slave
            .spawn_command(cmd)
            .expect("Couldn't spawn nvim command");

        let mut pty_reader = pair.master
            .try_clone_reader()
            .expect("Couldn't clone reader");

        let mut pty_buffer = [0u8; 2048];
        
        let mut pty_writer = pair.master
            .take_writer()
            .expect("Can't take PTY writer");

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut buffer = [0; 2048];
            loop {
                match pty_reader.read(&mut buffer) {
                    Ok(size) => {
                        if size > 0 {
                            let data = buffer[..size].to_vec();
                            tx.send(data).expect("Failed to sned data");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading: {:?}", e);
                        break;
                    }
                }
            }
        });

        Self {
            pty_system,
            pty_pair: pair,
            rx,
            writer: pty_writer,
            buffer: pty_buffer,
        }
    }
}

impl Element for PtyView {
    fn output(&mut self, stdout: &mut Stdout) -> std::io::Result<()> { 
        match self.rx.try_recv() {
            Ok(data) => {
                stdout.write(data.as_slice())?;
                stdout.flush()?;
                Ok(())
            },
            Err(TryRecvError::Empty) => Ok(()),
            Err(TryRecvError::Disconnected) => Ok(()),
        }
    }

    fn on_event(&mut self, event: &Event) -> std::io::Result<()> {
        if let Event::Key(key_event) = event {
            let input_bytes = match key_event.code {
                KeyCode::Char(c) => {
                    let mut bytes = Vec::new();

                    if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                        bytes.extend_from_slice(
                            &c
                            .to_uppercase()
                            .next()
                            .unwrap()
                            .to_string()
                            .as_bytes()
                        );
                    } else {
                        bytes.extend_from_slice(&c.to_string().as_bytes());
                    }

                    bytes
                },
                _ => {
                    Vec::new()
                },
            };

            self.writer.write(&input_bytes)
                .expect("Couldn't write");
        }

        Ok(())
    }
}
