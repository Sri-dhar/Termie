use crate::command::execute_command;
use crate::file_operations;
use crate::gemini_integration::call_gemini;
use crate::command::CommandInstance;    

use core::f32;
use eframe::egui;
use egui::{Key, ScrollArea, TopBottomPanel};
use std::env;
use std::fs;
use std::sync::mpsc;

pub struct MyApp {
    show_error: bool,
    command_input: String,
    last_ran_cmd: String,
    send_button_pressed: bool,
    commands: Vec<CommandInstance>,
    text_area_id: Option<egui::Id>,
    request_focus_next_frame: bool,
    gemini_response: String,
    gemini_input: String,
    file_bash_history: std::fs::File,
    file_history_arrows: std::fs::File,
    arrow_index: i32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            show_error: true,
            command_input: String::new(),
            last_ran_cmd: String::new(),
            send_button_pressed: false,
            commands: Vec::new(),
            text_area_id: None,
            request_focus_next_frame: false,
            gemini_response: String::new(),
            gemini_input: String::new(),
            file_bash_history: fs::OpenOptions::new()
                .append(true)
                .open("./history/.bash_history")
                .expect("Failed to open bash history file"),
            file_history_arrows: fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open("./history/.lastXcmds")
                .expect("Failed to open history file"),
            arrow_index: 501,
        }
    }
}

impl MyApp {
    fn custom_command_handling(&mut self, command: String) -> bool {
        let args: Vec<&str> = command.split_whitespace().collect();
        let (cmd, _rest) = args.split_at(1);
        let cmd = cmd[0]; // status: String,

        let _rest = _rest.join(" ");

        if cmd == "clear" {
            self.commands.clear();
            self.command_input.clear();
            return true;
        }

        if cmd == "exit" {
            std::process::exit(0);
        }

        false
    }

    fn print_command_with_time(&self, command: &str) {
        let def_tim = String::from(file_operations::get_current_time());
        let time = self
            .commands
            .last()
            .map(|cmd| &cmd.time)
            .unwrap_or(&def_tim);
        println!("=> {} {}", time, command);
    }

    fn handle_send_command(&mut self) {
        self.print_command_with_time(&self.command_input);
        file_operations::write_to_bash_history(&mut self.file_bash_history, self.command_input.clone());

        if self.command_input.is_empty() {
            self.send_button_pressed = false;
            return;
        }
        if self.custom_command_handling(self.command_input.clone()) {
            self.send_button_pressed = false;
            return;
        }

        let mut cwd = env::current_dir().unwrap();

        let output = match execute_command(&self.command_input, &mut cwd) {
            Ok(output) => output,
            Err(e) => e.to_string(),
        };
        self.commands.push(CommandInstance {
            counter: self.commands.len() as i32,
            command: self.command_input.clone(),
            output: output,
            time: file_operations::get_current_time().to_string(),
        });

        self.command_input.clear();
        self.send_button_pressed = false;
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut _show_error = self.show_error;
        if _show_error {
            ctx.set_pixels_per_point(1.5);

            // Top panel
            TopBottomPanel::top("Top Panel").show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(">> ");
                    ui.monospace(self.last_ran_cmd.clone());
                    ui.add_space(10.0);
                });
            });

            // Central panel with scroll area
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical(|ui| {
                    ScrollArea::vertical()
                        .max_width(f32::INFINITY)
                        .animated(true)
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            for command in &self.commands {
                                ui.separator();
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("{}", command.counter));
                                        ui.add_space(3.0);
                                        ui.horizontal_wrapped(|ui| {
                                            ui.label(format!("{}", command.command));
                                            ui.add_space(3.0);
                                        });
                                    });
                                    ui.label(format!("{}", command.output));
                                });
                            }
                            ui.add_space(40.0);
                        });
                });
            });

            // Bottom panel
            TopBottomPanel::bottom("Bottom Panel").show(ctx, |ui| {
                // showing current directory path
                ui.monospace(format!("{}: ", env::current_dir().unwrap().display()));

                ui.horizontal(|ui| {
                    let text_edit = egui::TextEdit::singleline(&mut self.command_input)
                        .desired_width(ui.available_width() - 129.2);
                    let text_area = ui.add(text_edit);

                    self.text_area_id = Some(text_area.id);

                    let send_button = ui.button("Send").on_hover_text("Send");
                    let ai_button = ui.button("Ask Gemini").on_hover_text("Ask Gemini");

                    if send_button.clicked() || self.send_button_pressed {
                        self.arrow_index = 501;
                        let _response = file_operations::append_string_to_file(
                            &mut self.file_history_arrows,
                            self.command_input.clone(),
                        );
                        self.handle_send_command();
                    }

                    if ai_button.clicked() {
                        self.gemini_input = self.command_input.clone();
                        let gemini_input = self.gemini_input.clone();
                        let ctx = ctx.clone();
                        let (tx, rx) = mpsc::channel::<String>();

                        tokio::spawn(async move {
                            match call_gemini(gemini_input).await {
                                Ok(response) => {
                                    // Update the application state with the response
                                    // If you need to update any state in `self`, consider adding a channel to pass the result back
                                    // println!("Gemini Response: {}", response);
                                    match tx.send(response) {
                                        Ok(_) => println!("Data sent successfully"),
                                        Err(e) => eprintln!("Failed to send data: {}", e),
                                    }
                                    ctx.request_repaint();
                                }
                                Err(e) => {
                                    eprintln!("Error: {}", e);
                                }
                            }
                        });

                        match rx.recv() {
                            Ok(response) => {
                                self.gemini_response = response;
                                self.command_input = self.gemini_response.clone();
                            }
                            Err(e) => {
                                eprintln!("Error: {}", e);
                            }
                        }
                    }

                    if ctx.input(|i| i.key_pressed(Key::Enter)) {
                        self.last_ran_cmd = self.command_input.clone();
                        self.send_button_pressed = true;
                        self.request_focus_next_frame = true; // Set a flag to request focus in the next frame
                    }

                    if self.arrow_index != 0 && ctx.input(|i| i.key_pressed(Key::ArrowUp)) {
                        if self.arrow_index > 0 {
                            self.arrow_index -= 1;
                        }
                        let buffer =
                        file_operations::get_string_from_file(&mut self.file_history_arrows, self.arrow_index);

                        println!("Up arrow pressed");
                        self.command_input = buffer;
                    }

                    if self.arrow_index != 501 && ctx.input(|i| i.key_pressed(Key::ArrowDown)) {
                        if self.arrow_index < 501 {
                            self.arrow_index += 1;
                        }
                        let buffer =
                        file_operations::get_string_from_file(&mut self.file_history_arrows, self.arrow_index);

                        println!("Down arrow pressed");
                        self.command_input = buffer;
                    }
                });
            });

            if self.request_focus_next_frame {
                if let Some(id) = self.text_area_id {
                    ctx.memory_mut(|memory| memory.request_focus(id));
                    println!("Down arrow pressed");
                }
                self.request_focus_next_frame = false;
            }
        }
    }
}
