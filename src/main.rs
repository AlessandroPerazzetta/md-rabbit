use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use std::env;
use std::fs;

#[derive(PartialEq)]
enum Mode {
    View,
    Edit,
}

struct MarkdownViewer {
    markdown: String,
    cache: CommonMarkCache,
    mode: Mode,
    file_path: Option<String>,
}

impl Default for MarkdownViewer {
    fn default() -> Self {
        Self {
            markdown: "No file loaded. Use File > Open to load a Markdown file.".to_string(),
            cache: CommonMarkCache::default(),
            mode: Mode::View,
            file_path: None,
        }
    }
}

impl MarkdownViewer {
    fn new(initial_markdown: String, file_path: Option<String>) -> Self {
        Self {
            markdown: initial_markdown,
            cache: CommonMarkCache::default(),
            mode: Mode::View,
            file_path,
        }
    }
}

impl eframe::App for MarkdownViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Markdown", &["md"])
                            .pick_file()
                        {
                            match load_markdown(path.to_str().unwrap()) {
                                Ok(markdown) => {
                                    self.markdown = markdown;
                                    self.file_path = Some(path.to_str().unwrap().to_string());
                                    self.mode = Mode::View;
                                }
                                Err(e) => self.markdown = format!("Failed to load file: {}", e),
                            }
                        }
                    }
                    if ui.button("Save").clicked() && self.file_path.is_some() {
                        if let Err(e) = save_markdown(self.file_path.as_ref().unwrap(), &self.markdown) {
                            eprintln!("Failed to save file: {}", e);
                        }
                    }
                });
                if ui.button(if self.mode == Mode::View { "Edit" } else { "View" }).clicked() {
                    self.mode = if self.mode == Mode::View { Mode::Edit } else { Mode::View };
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.mode {
                Mode::View => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        CommonMarkViewer::new().show(ui, &mut self.cache, &self.markdown);
                    });
                }
                Mode::Edit => {
                    ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.markdown));
                }
            }
        });
    }
}

fn load_markdown(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn save_markdown(path: &str, content: &str) -> Result<(), std::io::Error> {
    fs::write(path, content)
}

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = env::args().collect();
    let app = if args.len() > 1 {
        let file_path = &args[1];
        match load_markdown(file_path) {
            Ok(markdown) => MarkdownViewer::new(markdown, Some(file_path.to_string())),
            Err(e) => MarkdownViewer::new(format!("Failed to load file '{}': {}", file_path, e), None),
        }
    } else {
        MarkdownViewer::default()
    };

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Markdown Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}
