use eframe::egui;
use crate::app::Myapp;
use eframe::egui::{RichText, Color32};

pub fn render(app: &mut Myapp, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.heading("演出项目列表");
        ui.add_space(10.0);
        
        // 搜索和筛选区域
        ui.horizontal(|ui| {
            ui.label("搜索：");
            let search_response = ui.text_edit_singleline(&mut app.search_keyword);
            
            if search_response.changed() {
                app.filter_projects();
            }
            
            ui.separator();
            
            // 状态筛选
            ui.label("状态：");
            egui::ComboBox::from_id_source("status_filter")
                .selected_text(app.filter_status.as_deref().unwrap_or("全部"))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut app.filter_status, None, "全部").clicked() {
                        app.filter_projects();
                    }
                    for status in &["售票中", "即将开售", "已售罄", "已结束", "已取消"] {
                        if ui.selectable_value(&mut app.filter_status, Some(status.to_string()), *status).clicked() {
                            app.filter_projects();
                        }
                    }
                });
                
            // 城市筛选
            ui.label("城市：");
            egui::ComboBox::from_id_source("city_filter")
                .selected_text(app.filter_city.as_deref().unwrap_or("全部"))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut app.filter_city, None, "全部").clicked() {
                        app.filter_projects();
                    }
                    
                    // 从所有项目中提取城市列表
                    let mut cities = app.all_projects.iter()
                        .map(|p| p.city.clone())
                        .collect::<std::collections::HashSet<_>>()
                        .into_iter()
                        .collect::<Vec<_>>();
                    cities.sort();
                    
                    for city in cities {
                        if ui.selectable_value(&mut app.filter_city, Some(city.clone()), city).clicked() {
                            app.filter_projects();
                        }
                    }
                });
                
            // 刷新按钮
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🔄 刷新数据").clicked() {
                    app.refresh_projects();
                }
            });
        });
        
        ui.add_space(5.0);
        
        // 价格筛选
        ui.horizontal(|ui| {
            ui.label("价格：");
            
            // 最低价格
            ui.label("从");
            let mut min_price = app.filter_price_min.unwrap_or(0.0);
            if ui.add(egui::DragValue::new(&mut min_price)
                .speed(10.0)
                .clamp_range(0.0..=10000.0)
                .prefix("￥")
            ).changed() {
                app.filter_price_min = Some(min_price);
                app.filter_projects();
            }
            if ui.button("清除").clicked() {
                app.filter_price_min = None;
                app.filter_projects();
            }
            
            // 最高价格
            ui.label("到");
            let mut max_price = app.filter_price_max.unwrap_or(10000.0);
            if ui.add(egui::DragValue::new(&mut max_price)
                .speed(10.0)
                .clamp_range(0.0..=10000.0)
                .prefix("￥")
            ).changed() {
                app.filter_price_max = Some(max_price);
                app.filter_projects();
            }
            if ui.button("清除").clicked() {
                app.filter_price_max = None;
                app.filter_projects();
            }
            
            // 排序选项
            ui.separator();
            ui.label("排序：");
            egui::ComboBox::from_id_source("sort_option")
                .selected_text(&app.selected_sort_option)
                .show_ui(ui, |ui| {
                    for option in &["更新时间", "开始时间", "价格", "项目名称"] {
                        if ui.selectable_value(&mut app.selected_sort_option, option.to_string(), *option).clicked() {
                            app.sort_projects();
                            app.update_page_projects();
                        }
                    }
                });
                
            // 升序/降序
            if ui.selectable_label(app.sort_ascending, "↑").clicked() {
                app.sort_ascending = true;
                app.sort_projects();
                app.update_page_projects();
            }
            if ui.selectable_label(!app.sort_ascending, "↓").clicked() {
                app.sort_ascending = false;
                app.sort_projects();
                app.update_page_projects();
            }
        });
        
        ui.add_space(10.0);
        
        // 项目列表表头
        ui.horizontal(|ui| {
            ui.add_sized(egui::vec2(300.0, 0.0), egui::Label::new(RichText::new("项目名称").strong()));
            ui.add_sized(egui::vec2(120.0, 0.0), egui::Label::new(RichText::new("地点").strong()));
            ui.add_sized(egui::vec2(60.0, 0.0), egui::Label::new(RichText::new("场次").strong()));
            ui.add_sized(egui::vec2(120.0, 0.0), egui::Label::new(RichText::new("价格").strong()));
            ui.add_sized(egui::vec2(180.0, 0.0), egui::Label::new(RichText::new("开始时间").strong()));
            ui.add_sized(egui::vec2(80.0, 0.0), egui::Label::new(RichText::new("状态").strong()));
            ui.add_sized(egui::vec2(80.0, 0.0), egui::Label::new(RichText::new("操作").strong()));
        });
   
        ui.separator();
        
        // 项目列表内容
        egui::ScrollArea::vertical().show(ui, |ui| {
            if app.current_page_projects.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label("没有找到符合条件的项目");
                });
            } else {
                for project in &app.current_page_projects {
                    ui.horizontal(|ui| {
                        // 项目名称
                        ui.add_sized(egui::vec2(300.0, 0.0), egui::Label::new(&project.name));
                        
                        // 地点
                        ui.add_sized(
                            egui::vec2(120.0, 0.0),
                            egui::Label::new(format!("{} {}", project.city, project.venue))
                        );
                        
                        // 场次
                        ui.add_sized(
                            egui::vec2(60.0, 0.0),
                            egui::Label::new(format!("{}场", project.screen_count))
                        );
                        
                        // 价格
                        ui.add_sized(
                            egui::vec2(120.0, 0.0),
                            egui::Label::new(format!("￥{} - ￥{}", 
                                project.price_low as i32, 
                                project.price_high as i32))
                        );
                        
                        // 开始时间
                        ui.add_sized(
                            egui::vec2(180.0, 0.0),
                            egui::Label::new(&project.start_time[..16])
                        );
                        
                        // 状态
                        let status_color = match project.status.as_str() {
                            "售票中" => Color32::from_rgb(76, 175, 80),
                            "即将开售" => Color32::from_rgb(33, 150, 243),
                            "已售罄" => Color32::from_rgb(255, 152, 0),
                            "已结束" => Color32::from_rgb(158, 158, 158),
                            "已取消" => Color32::from_rgb(244, 67, 54),
                            _ => ui.visuals().text_color(),
                        };
                        
                        ui.add_sized(
                            egui::vec2(80.0, 0.0),
                            egui::Label::new(RichText::new(&project.status).color(status_color))
                        );
                        
                        // 详情按钮
                        if ui.button("详情").clicked() {
                            app.selected_project_id = Some(project.id.clone());
                            app.show_project_details = true;
                        }
                    });
                    
                    ui.separator();
                }
            }
        });
        
        // 分页控件
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label(format!("共 {} 条记录，当前第 {} / {} 页", 
                app.total_projects, 
                app.current_page,
                app.total_pages));
            
            ui.separator();
            
            // 首页
            if ui.add_enabled(
                app.current_page > 1, 
                egui::Button::new("<<")
            ).clicked() {
                app.current_page = 1;
                app.update_page_projects();
            }
            
            // 上一页
            if ui.add_enabled(
                app.current_page > 1, 
                egui::Button::new("< 上一页")
            ).clicked() {
                app.current_page -= 1;
                app.update_page_projects();
            }
            
            // 页码选择
            let mut current_page = app.current_page as i32;
            if ui.add(egui::DragValue::new(&mut current_page)
                .speed(1.0)
                .clamp_range(1..=app.total_pages as i32)
            ).changed() {
                app.current_page = current_page as usize;
                app.update_page_projects();
            }
            
            // 下一页
            if ui.add_enabled(
                app.current_page < app.total_pages, 
                egui::Button::new("下一页 >")
            ).clicked() {
                app.current_page += 1;
                app.update_page_projects();
            }
            
            // 末页
            if ui.add_enabled(
                app.current_page < app.total_pages, 
                egui::Button::new(">>")
            ).clicked() {
                app.current_page = app.total_pages;
                app.update_page_projects();
            }
            
            ui.separator();
            
            // 每页显示数量
            ui.label("每页显示：");
            egui::ComboBox::from_id_source("page_size")
                .selected_text(format!("{}", app.page_size))
                .show_ui(ui, |ui| {
                    for size in [10, 20, 50, 100] {
                        if ui.selectable_value(&mut app.page_size, size, format!("{}", size)).clicked() {
                            app.total_pages = (app.total_projects + app.page_size - 1) / app.page_size;
                            app.current_page = 1;
                            app.update_page_projects();
                        }
                    }
                });
        });
    });
    
    // 项目详情弹窗
    if app.show_project_details {
        show_project_details(app);
    }

}

// 项目详情弹窗
fn show_project_details(app: &mut Myapp) {
    let ctx = egui::Context::default(); // 创建一个新的Context或从app中获取
    
    let selected_id = match &app.selected_project_id {
        Some(id) => id.clone(),
        None => {
            app.show_project_details = false;
            return;
        }
    };
    
    // 查找选中的项目
    let project = match app.all_projects.iter().find(|p| p.id == selected_id) {
        Some(p) => p.clone(),
        None => {
            app.show_project_details = false;
            return;
        }
    };
    
    // 创建一个临时变量来跟踪窗口是否打开
    let mut show_window = true;
    // 用于记录关闭按钮的点击状态
    let mut close_clicked = false;
    
    egui::Window::new(format!("项目详情 - {}", project.name))
        .open(&mut show_window)
        .resizable(true)
        .default_size([600.0, 400.0])
        .show(&ctx, |ui| {
            ui.vertical(|ui| {
                // 基本信息区域
                ui.heading(&project.name);
                ui.label(&project.description);
                ui.add_space(10.0);
                
                // 项目信息卡片
                egui::Frame::none()
                    .fill(ui.visuals().extreme_bg_color)
                    .rounding(5.0)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.strong("演出场馆");
                                ui.label(format!("{} {}", project.city, project.venue));
                                ui.add_space(5.0);
                                ui.strong("价格区间");
                                ui.label(format!("￥{} - ￥{}", project.price_low as i32, project.price_high as i32));
                            });
                            
                            ui.separator();
                            
                            ui.vertical(|ui| {
                                ui.strong("开始时间");
                                ui.label(&project.start_time);
                                ui.add_space(5.0);
                                ui.strong("结束时间");
                                ui.label(&project.end_time);
                            });
                            
                            ui.separator();
                            
                            ui.vertical(|ui| {
                                ui.strong("项目状态");
                                let status_color = match project.status.as_str() {
                                    "售票中" => Color32::from_rgb(76, 175, 80),
                                    "即将开售" => Color32::from_rgb(33, 150, 243),
                                    "已售罄" => Color32::from_rgb(255, 152, 0),
                                    "已结束" => Color32::from_rgb(158, 158, 158),
                                    "已取消" => Color32::from_rgb(244, 67, 54),
                                    _ => ui.visuals().text_color(),
                                };
                                ui.label(RichText::new(&project.status).color(status_color).strong());
                                ui.add_space(5.0);
                                ui.strong("更新时间");
                                ui.label(&project.update_time);
                            });
                        });
                    });
                
                ui.add_space(20.0);
                
                // 场次信息标题
                ui.heading("场次信息");
                ui.separator();
                
                // 生成一些随机场次数据
                let screen_count = project.screen_count as usize;
                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        // 为简化，这里生成一些示例场次数据
                        let base_date = chrono::NaiveDateTime::parse_from_str(
                            &project.start_time, "%Y-%m-%d %H:%M:%S").unwrap_or_default();
                            
                        for i in 0..screen_count {
                            let screen_date = base_date + chrono::Duration::days(i as i64);
                            let screen_end = screen_date + chrono::Duration::hours(2);
                            
                            let status = match i % 5 {
                                0 => "售票中",
                                1 => "即将开售",
                                2 => "已售罄", 
                                3 => "已结束",
                                _ => "已取消",
                            };
                            
                            let status_color = match status {
                                "售票中" => Color32::from_rgb(76, 175, 80),
                                "即将开售" => Color32::from_rgb(33, 150, 243),
                                "已售罄" => Color32::from_rgb(255, 152, 0),
                                "已结束" => Color32::from_rgb(158, 158, 158),
                                "已取消" => Color32::from_rgb(244, 67, 54),
                                _ => ui.visuals().text_color(),
                            };
                            
                            ui.horizontal(|ui| {
                                ui.add_sized(
                                    egui::vec2(80.0, 0.0),
                                    egui::Label::new(format!("场次 {}", i+1))
                                );
                                ui.add_sized(
                                    egui::vec2(180.0, 0.0),
                                    egui::Label::new(screen_date.format("%Y-%m-%d %H:%M").to_string())
                                );
                                ui.add_sized(
                                    egui::vec2(60.0, 0.0),
                                    egui::Label::new(screen_end.format("%H:%M").to_string())
                                );
                                ui.add_sized(
                                    egui::vec2(80.0, 0.0),
                                    egui::Label::new(RichText::new(status).color(status_color))
                                );
                                
                                if status == "售票中" {
                                    if ui.button("购票").clicked() {
                                        // 购票逻辑
                                    }
                                }
                            });
                            ui.separator();
                        }
                    });
                
                ui.add_space(10.0);
                    
                // 底部按钮
                ui.vertical_centered(|ui| {
                    if ui.button("关闭").clicked() {
                        // 不直接修改show_window，而是设置一个标志
                        close_clicked = true;
                    }
                });
            });
        });
    
    // 在窗口闭包结束后更新app的状态
    // 这里同时检查窗口是否被关闭或关闭按钮是否被点击
    if !show_window || close_clicked {
        app.show_project_details = false;
    }
}