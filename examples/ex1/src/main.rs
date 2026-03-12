//! Show a custom window frame instead of the default OS window chrome decorations.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::sync::mpsc::{Receiver, Sender};
use eframe::egui::{self, Theme, ViewportCommand,
                  Color32, Stroke, Style, global_theme_preference_buttons, style::Selection,
};
use egui_demo_lib::{View as _, WidgetGallery};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    /*let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false) // Hide the OS-specific "chrome" around the window
            .with_inner_size([900.0, 600.0])
            .with_min_inner_size([870.0, 370.0])
            .with_resizable(true)
            .with_transparent(true), // To have rounded corners we need transparency

        ..Default::default()
    };*/
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([350.0, 590.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Выборочное издание", // unused title
        options,
        Box::new(|cc| Ok(Box::new(Введение::new(cc)))),
    )
    /*println!("поехало");
    println!("поехало2");*/
}



fn setup_custom_style(ctx: &egui::Context) {
    ctx.style_mut_of(Theme::Light, use_light_green_accent);
    ctx.style_mut_of(Theme::Dark, use_dark_purple_accent);
}

fn use_light_green_accent(style: &mut Style) {
    style.visuals.hyperlink_color = Color32::from_rgb(18, 180, 85);
    style.visuals.text_cursor.stroke.color = Color32::from_rgb(28, 92, 48);
    style.visuals.selection = Selection {
        bg_fill: Color32::from_rgb(157, 218, 169),
        stroke: Stroke::new(1.0, Color32::from_rgb(28, 92, 48)),
    };
}

fn use_dark_purple_accent(style: &mut Style) {
    style.visuals.hyperlink_color = Color32::from_rgb(202, 135, 227);
    style.visuals.text_cursor.stroke.color = Color32::from_rgb(234, 208, 244);
    style.visuals.selection = Selection {
        bg_fill: Color32::from_rgb(105, 67, 119),
        stroke: Stroke::new(1.0, Color32::from_rgb(234, 208, 244)),
    };
}

struct Введение {
    name: String,
    isp_count: u32,
    age: u32,
    range_7: String,
    visible: bool,                 //видимость
    complect_reveal: bool,         //раскрыть комплекты
    bom_spec_rpt_path: String,     //путь до bom_spec.rpt
    niisi_conceptlib_path: String, //путь до niisi_conceptlib
    text: String,
    main_status: String, //состояние
    //файл
    text_channel: (Sender<String>, Receiver<String>),
    text_channel_2: (Sender<String>, Receiver<String>),
    sample_text: String,
    sample_text_2: String,
    button_1:String,//кнопка проверка данных
    button_2:String,//помощь
    button_3:String,//новый проект
    s1:String,//содержимое нижнего окна
    //custom_collapsing_header: CustomCollapsingHeader,
    en_button:bool,
    widget_gallery: WidgetGallery,
}

impl Введение {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_style(&cc.egui_ctx);
        egui_extras::install_image_loaders(&cc.egui_ctx); // Needed for the "Widget Gallery" demo
        Self {
            widget_gallery: Default::default(),
            // enabled: true,
            visible: true,
            range_7: "".to_string(),
            name: "".to_string(),
            isp_count: 1,
            age: 1,
            complect_reveal: false,
            en_button:false,
            bom_spec_rpt_path: r#"UNKNOWN"#.to_string(),
            niisi_conceptlib_path: r#"C:\PCB\niisi_elberi\niisi_conceptlib\niisi_conceptlib_list.xls"#.to_string(),
            text: "Нет текста".to_string(),
            main_status: "ЗАПУСК".to_string(),
            button_1:"Проверка данных".to_string(),
            button_2:"Помощь".to_string(),
            button_3:"Новый проект".to_string(),
            //custom_collapsing_header: Default::default(),
            // opacity: 1.0,
            // boolean: false,
            // radio: Enum::First,
            // scalar: 42.0,
            // string: Default::default(),
            // color: egui::Color32::LIGHT_BLUE.linear_multiply(0.5),
            // animate_progress_bar: false,
            //  #[cfg(feature = "chrono")]
            // date: None,
            // #[cfg(feature = "chrono")]
            // with_date_button: true,
            //файл
            text_channel: std::sync::mpsc::channel(),
            text_channel_2: std::sync::mpsc::channel(),
            sample_text: "This is some sample text".into(),
            sample_text_2: "This is some sample text".into(),
            s1:r#"Инициализация.
- Файл конфигурации не найден, создан по умолчанию.
- Настройка программы успешно завершена.
- Настройки сохранены.
Проверка исходных данных.
- Файл BOM_spec.rpt успешно загружен.
- Поиск PART_NUMBER по niisi_conceptlb_list.xls.
- Поиск комплектов PART_NUMBER по niisi_conceptlb_list.xls.
- Актуализация BOM_spec.rpt по базе компонентов выполнена успешно."#.to_string(),
        }
    }
}

impl eframe::App for Введение {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("egui using a customized style");
            ui.label("Switch between dark and light mode to see the different styles in action.");
            global_theme_preference_buttons(ui);
            ui.separator();
            self.widget_gallery.ui(ui);
        });
    }
}
/*
impl eframe::App for Введение {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        custom_window_frame(ui, "egui избирательное показание", |ui| {
            ui.label("This is just the contents of the window.");
            ui.horizontal(|ui| {
                ui.label("egui theme:");
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
    }
}

fn custom_window_frame(ui: &mut egui::Ui, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    use egui::UiBuilder;

    let panel_frame = egui::Frame::new()
        .fill(ui.global_style().visuals.window_fill())
        .corner_radius(10)
        .stroke(ui.global_style().visuals.widgets.noninteractive.fg_stroke)
        .outer_margin(1); // so the stroke is within the bounds

    panel_frame.show(ui, |ui| {
        let app_rect = ui.max_rect();

        ui.expand_to_include_rect(app_rect); // Expand frame to include it all

        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        title_bar_ui(ui, title_bar_rect, title);

        // Add the contents:
        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
        .shrink(4.0);
        let mut content_ui = ui.new_child(UiBuilder::new().max_rect(content_rect));
        add_contents(&mut content_ui);
    });
}

fn title_bar_ui(ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
    use egui::{Align2, FontId, Id, PointerButton, Sense, UiBuilder, vec2};

    let painter = ui.painter();

    let title_bar_response = ui.interact(
        title_bar_rect,
        Id::new("title_bar"),
        Sense::click_and_drag(),
    );

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
    }

    if title_bar_response.drag_started_by(PointerButton::Primary) {
        ui.send_viewport_cmd(ViewportCommand::StartDrag);
    }

    ui.scope_builder(
        UiBuilder::new()
            .max_rect(title_bar_rect)
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
        |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui);
        },
    );
}

/// Show some close/maximize/minimize buttons for the native window.
fn close_maximize_minimize(ui: &mut egui::Ui) {
    use egui::{Button, RichText};

    let button_height = 12.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        ui.send_viewport_cmd(egui::ViewportCommand::Close);
    }

    let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    if is_maximized {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Restore window");
        if maximized_response.clicked() {
            ui.send_viewport_cmd(ViewportCommand::Maximized(false));
        }
    } else {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Maximize window");
        if maximized_response.clicked() {
            ui.send_viewport_cmd(ViewportCommand::Maximized(true));
        }
    }

    let minimized_response = ui
        .add(Button::new(RichText::new("🗕").size(button_height)))
        .on_hover_text("Minimize the window");
    if minimized_response.clicked() {
        ui.send_viewport_cmd(ViewportCommand::Minimized(true));
    }
}
impl Введение {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_style(&cc.egui_ctx);
        egui_extras::install_image_loaders(&cc.egui_ctx); // Needed for the "Widget Gallery" demo
        Self {
            widget_gallery: Default::default(),
            // enabled: true,
            visible: true,
            range_7: "".to_string(),
            name: "".to_string(),
            isp_count: 1,
            age: 1,
            complect_reveal: false,
            en_button:false,
            bom_spec_rpt_path: r#"UNKNOWN"#.to_string(),
            niisi_conceptlib_path: r#"C:\PCB\niisi_elberi\niisi_conceptlib\niisi_conceptlib_list.xls"#.to_string(),
            text: "Нет текста".to_string(),
            main_status: "ЗАПУСК".to_string(),
            button_1:"Проверка данных".to_string(),
            button_2:"Помощь".to_string(),
            button_3:"Новый проект".to_string(),
            //custom_collapsing_header: Default::default(),
            // opacity: 1.0,
            // boolean: false,
            // radio: Enum::First,
            // scalar: 42.0,
            // string: Default::default(),
            // color: egui::Color32::LIGHT_BLUE.linear_multiply(0.5),
            // animate_progress_bar: false,
            //  #[cfg(feature = "chrono")]
            // date: None,
            // #[cfg(feature = "chrono")]
            // with_date_button: true,
            //файл
            text_channel: std::sync::mpsc::channel(),
            text_channel_2: std::sync::mpsc::channel(),
            sample_text: "This is some sample text".into(),
            sample_text_2: "This is some sample text".into(),
            s1:r#"Инициализация.
- Файл конфигурации не найден, создан по умолчанию.
- Настройка программы успешно завершена.
- Настройки сохранены.
Проверка исходных данных.
- Файл BOM_spec.rpt успешно загружен.
- Поиск PART_NUMBER по niisi_conceptlb_list.xls.
- Поиск комплектов PART_NUMBER по niisi_conceptlb_list.xls.
- Актуализация BOM_spec.rpt по базе компонентов выполнена успешно."#.to_string(),
        }
    }
}

fn setup_custom_style(ctx: &egui::Context) {
    ctx.style_mut_of(Theme::Light, use_light_green_accent);
    ctx.style_mut_of(Theme::Dark, use_dark_purple_accent);
}

fn use_light_green_accent(style: &mut Style) {
    style.visuals.hyperlink_color = Color32::from_rgb(18, 180, 85);
    style.visuals.text_cursor.stroke.color = Color32::from_rgb(28, 92, 48);
    style.visuals.selection = Selection {
        bg_fill: Color32::from_rgb(157, 218, 169),
        stroke: Stroke::new(1.0, Color32::from_rgb(28, 92, 48)),
    };
}
fn use_dark_purple_accent(style: &mut Style) {
    style.visuals.hyperlink_color = Color32::from_rgb(202, 135, 227);
    style.visuals.text_cursor.stroke.color = Color32::from_rgb(234, 208, 244);
    style.visuals.selection = Selection {
        bg_fill: Color32::from_rgb(105, 67, 119),
        stroke: Stroke::new(1.0, Color32::from_rgb(234, 208, 244)),
    };
}*/