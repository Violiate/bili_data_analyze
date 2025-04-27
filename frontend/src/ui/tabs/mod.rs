pub mod home;
pub mod monitor;
pub mod account;
pub mod settings;
pub mod help; 


pub mod dashboard;
pub mod project_list;
pub mod data_analysis;

use eframe::egui;
use crate::app::Myapp;


pub fn render_tab_content(app: &mut Myapp, ui: &mut egui::Ui) {
    match app.selected_tab {
        0 => dashboard::render_dashboard(app, ui),
        1 => project_list::render(app, ui),
        2 => data_analysis::render(app, ui),
        3 => settings::render(app, ui),
        4 => help::render(app, ui), 
        _ => unreachable!(),
    }
}