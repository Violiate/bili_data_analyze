use eframe::egui::{self, Ui, RichText, Color32, Stroke, Align, Frame, Rounding, Vec2};
use crate::app::Myapp;

fn create_modern_button(ui: &mut egui::Ui, text: &str, icon: &str, accent_color: egui::Color32) -> bool {
    let padding = egui::vec2(16.0, 12.0);
    let total_extra = padding + padding;
    
    // 创建富文本标签，包含图标和文字
    let rich_text = RichText::new(format!("{} {}", icon, text))
        .size(18.0)
        .color(egui::Color32::WHITE)
        .strong();
    
        let button_size = egui::vec2(
            ui.available_width().min(240.0),  // 宽度适应但不超过240
            48.0  // 固定高度48
        );
    
    // 构建按钮
    let mut button = egui::Button::new(rich_text)
        .min_size(button_size)
        .fill(accent_color)
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(220)))
        .rounding(12.0); // 使用较大圆角
    
    // 添加按钮到UI
    let response = ui.add(button);
    
    // 悬停效果
    if response.hovered() {
        // 鼠标悬停时显示阴影效果
        let hover_rect = response.rect.expand(2.0);
        ui.painter().rect_stroke(
            hover_rect, 
            12.0, 
            egui::Stroke::new(2.0, accent_color.linear_multiply(1.2))
        );
    }
    
    response.clicked()
}
fn add_action_buttons(app: &mut Myapp, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(10.0);
        
        // 使用更合适的宽度设置
        ui.horizontal_centered(|ui| {
            ui.set_max_width(ui.available_width().min(600.0)); // 限制最大宽度
            
            let accent_color = egui::Color32::from_rgb(102, 204, 255);
            
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_main_justify(true), |ui| {
                // 登录按钮
                if create_modern_button(ui, "账号登录", "👤", accent_color) {
                    app.show_login_windows = true;
                }
                
                ui.add_space(20.0); // 按钮之间的间距
                
                // 日志按钮
                if create_modern_button(ui, "查看日志", "📋", accent_color) {
                    app.show_log_window = true;
                }
            });
        });
        
        ui.add_space(10.0);
    });
}

pub fn render_dashboard(app: &mut Myapp, ui: &mut egui::Ui) {
    // 设置整体背景和间距
    Frame::none()
        .inner_margin(20.0)
        .show(ui, |ui| {
            // 顶部区域：标题和操作按钮并排
            ui.horizontal(|ui| {
                // 左侧标题
                ui.vertical(|ui| {
                    ui.add_space(10.0);
                    ui.heading(RichText::new("B站演出数据概览").size(28.0).strong());
                    ui.add_space(5.0);
                    ui.label(RichText::new("实时监控B站演出市场动态").color(Color32::from_rgb(140, 140, 150)));
                });
                
                // 右侧操作按钮区
                ui.with_layout(egui::Layout::right_to_left(Align::TOP), |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(8.0);
                        
                        // 让按钮水平排列
                        ui.horizontal(|ui| {
                            let accent_color = egui::Color32::from_rgb(102, 204, 255);
                            
                            // 日志按钮
                            if create_modern_button(ui, "查看日志", "📋", accent_color) {
                                app.show_log_window = true;
                            }
                            
                            ui.add_space(10.0); // 按钮之间的间距
                            
                            // 登录按钮
                            if create_modern_button(ui, "账号登录", "👤", accent_color) {
                                app.show_login_windows = true;
                            }
                        });
                    });
                });
            });

            ui.add_space(20.0);
            
            // 数据统计卡片 - 使用现代卡片设计
            ui.horizontal_wrapped(|ui| {
                // 使用Layout::justified让项目均匀分布
                ui.with_layout(egui::Layout::left_to_right(Align::Center).with_cross_justify(true), |ui| {
                    // 强制水平布局占用所有宽度
                    let full_width = ui.available_width();
                    ui.set_min_width(full_width);
                    
                    // 计算每个卡片的宽度（考虑到内边距）
                    let card_width = (full_width - 32.0) / 3.0; // 减去间距
                    
                    render_modern_stat_card(ui, "总项目数", app.project_count, "📊", Color32::from_rgb(77, 129, 231), card_width);
                    ui.add_space(16.0);
                    render_modern_stat_card(ui, "本月新增", app.new_projects_count, "📈", Color32::from_rgb(94, 179, 129), card_width);
                    ui.add_space(16.0);
                    render_modern_stat_card(ui, "已完成项目", app.finished_projects_count, "✓", Color32::from_rgb(243, 154, 74), card_width);
                });
            });
            
            ui.add_space(24.0);
            
            // 最近趋势简要图表
            render_trend_section(ui, app);
            
            ui.add_space(24.0);
            
            // 最近更新项目列表 - 使用现代卡片设计
            Frame::none()
                .fill(ui.visuals().widgets.noninteractive.bg_fill)
                .rounding(Rounding::same(12.0))
                .inner_margin(Vec2::new(16.0, 16.0))
                .stroke(Stroke::new(1.0, Color32::from_gray(60)))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading("最近更新项目");
                        ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                            if ui.button(RichText::new("📥 刷新数据").text_style(egui::TextStyle::Button)).clicked() {
                                app.refresh_data_flag = true;
                            }
                        });
                    });
                    ui.add_space(8.0);

                    let mut selected_project_id: Option<String> = None;
                    
                    // 数据列表
                    egui::ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                        if app.recent_projects.is_empty() {
                            ui.label(RichText::new("暂无数据，请点击刷新按钮获取最新项目").color(Color32::from_gray(140)));
                        } else {
                            for (i, project) in app.recent_projects.iter().enumerate() {
                                // 修改render_project_item函数不再接收app参数，而是返回是否点击
                                if let Some(id) = render_project_item(ui, project, i) {
                                    selected_project_id = Some(id);
                                }
                                
                                if i < app.recent_projects.len() - 1 {
                                    ui.add_space(8.0);
                                }
                            }
                        }
                    });
                    
                    // 迭代结束后更新app状态
                    if let Some(id) = selected_project_id {
                        app.selected_project_id = Some(id);
                        app.show_project_details = true;
                    }
            });
            
            ui.add_space(10.0);
            
            // 数据最后更新时间
            ui.with_layout(egui::Layout::right_to_left(Align::RIGHT), |ui| {
                ui.label(RichText::new(format!("最后更新: {}", app.last_update_time)).color(Color32::from_rgb(130, 130, 140)));
            });
        });
}

// 绘制现代化统计卡片
fn render_modern_stat_card(ui: &mut egui::Ui, title: &str, value: usize, icon: &str, accent_color: Color32, card_width: f32) {
    // 使用固定高度确保一致性
    let card_height = 100.0;
    
    Frame::none()
        .fill(ui.visuals().widgets.noninteractive.bg_fill)
        .rounding(Rounding::same(12.0))
        .shadow(egui::epaint::Shadow {
            extrusion: 2.0,
            color: Color32::from_black_alpha(15),
        })
        .stroke(Stroke::new(1.0, Color32::from_gray(60)))
        .show(ui, |ui| {
            // 使用传入的固定宽度
            let desired_size = Vec2::new(card_width, card_height);
            ui.set_min_size(desired_size);
            ui.set_max_size(desired_size);
            
            // 使用水平布局确保图标和文本并排显示
            ui.horizontal(|ui| {
                // 左侧图标 - 使用固定尺寸的框
                let icon_size = 40.0;
                Frame::none()
                    .fill(accent_color.linear_multiply(0.2))
                    .rounding(Rounding::same(8.0))
                    .show(ui, |ui| {
                        ui.allocate_ui(Vec2::new(icon_size, icon_size), |ui| {
                            ui.centered_and_justified(|ui| {
                                ui.label(RichText::new(icon).size(24.0));
                            });
                        });
                    });

                ui.add_space(12.0);
                
                // 右侧文字区域 - 使用剩余的所有空间
                ui.vertical(|ui| {
                    ui.set_min_width(card_width - icon_size - 24.0);
                    ui.add_space(10.0);
                    ui.label(RichText::new(title).color(Color32::from_rgb(170, 170, 180)));
                    ui.add_space(4.0);
                    
                    // 为数字使用较大字体并确保水平显示
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(
                            RichText::new(format!("{}", value))
                                .color(accent_color)
                                .size(26.0)
                                .strong()
                        ));
                    });
                    
                    ui.add_space(10.0);
                });
            });
        });
}

// 绘制简单趋势图表
fn render_trend_section(ui: &mut egui::Ui, app: &mut Myapp) {
    Frame::none()
        .fill(ui.visuals().widgets.noninteractive.bg_fill)
        .rounding(Rounding::same(12.0))
        .inner_margin(Vec2::new(16.0, 16.0))
        .stroke(Stroke::new(1.0, Color32::from_gray(60)))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.heading("项目趋势");
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        egui::ComboBox::from_label("")
                            .selected_text("最近30天")
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut app.selected_time_range, "最近7天".to_string(), "最近7天");
                                ui.selectable_value(&mut app.selected_time_range, "最近30天".to_string(), "最近30天");
                                ui.selectable_value(&mut app.selected_time_range, "最近90天".to_string(), "最近90天");
                            });
                            
                        ui.label("时间范围：");
                    });
                });
                
                ui.add_space(8.0);
                
                // 简单趋势图表 - 这里我们绘制一个简单的条形图
                let chart_height = 120.0;
                let chart_width = ui.available_width();
                
                // 绘制图表区域
                let (response, painter) = ui.allocate_painter(Vec2::new(chart_width, chart_height), egui::Sense::hover());
                let rect = response.rect;
                
                // 填充月份数据 - 理想情况下从app中获取真实数据
                let months = ["1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月", "11月", "12月"];
                let values = get_monthly_values(app);
                
                

                let max_value = values.iter().copied().fold(0, i32::max);
                let total_bars = values.len() as f32;
                let padding = 10.0 * (total_bars - 1.0); // 所有条形图之间的总间距
                let bar_width = (rect.width() - padding) / total_bars - 2.0; // 减去2像素作为安全边距
                
                // 绘制背景网格线
                for i in 0..5 {
                    let y = rect.top() + (i as f32 * chart_height / 4.0);
                    painter.line_segment(
                        [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                        Stroke::new(1.0, Color32::from_gray(50))
                    );
                }
                
                // 绘制条形
                for (i, (&month, &value)) in months.iter().zip(values.iter()).enumerate() {
                    if value > 0 {
                        let bar_height = (value as f32 / max_value as f32) * chart_height;
                        let x = rect.left() + (i as f32 * (rect.width() / values.len() as f32)) + 5.0;
                        let bar_rect = egui::Rect::from_min_size(
                            egui::pos2(x, rect.bottom() - bar_height),
                            egui::vec2(bar_width, bar_height),
                        );
                        
                       
                        painter.rect_filled(
                            bar_rect,
                            Rounding::same(4.0),
                            Color32::from_rgb(148, 124, 240) // 使用中间色调
                        );
                        
                        
                        
                        // 绘制月份标签
                        painter.text(
                            egui::pos2(x + bar_width / 2.0, rect.bottom() + 10.0),
                            egui::Align2::CENTER_CENTER,
                            month,
                            egui::TextStyle::Body.resolve(ui.style()),
                            Color32::from_gray(180)
                        );
                    }
                }
            });
    });
}

// 获取月度数据值
fn get_monthly_values(app: &Myapp) -> Vec<i32> {
    // 优先使用app中的月度趋势数据
    if !app.monthly_trend.is_empty() {
        let mut values = vec![0; 12];
        for (month, value) in &app.monthly_trend {
            if *month >= 1 && *month <= 12 {
                values[*month as usize - 1] = *value;
            }
        }
        return values;
    }
    
    // 如果没有数据，返回模拟数据
    vec![23, 28, 32, 38, 45, 52, 48, 42, 50, 58, 63, 70]
}

// 绘制项目列表项
fn render_project_item(ui: &mut egui::Ui, project: &common::data_summary::ProjectSummary, index: usize) -> Option<String> {
    let mut clicked = false;
    Frame::none()
        .fill(if index % 2 == 0 { 
            ui.visuals().widgets.noninteractive.bg_fill
        } else { 
            ui.visuals().widgets.noninteractive.bg_fill.linear_multiply(1.05)
        })
        .rounding(Rounding::same(8.0))
        .inner_margin(Vec2::new(12.0, 12.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let status_color = match project.status.as_str() {
                            "售票中" => Color32::from_rgb(76, 175, 80),
                            "即将开售" => Color32::from_rgb(33, 150, 243),
                            "已售罄" => Color32::from_rgb(255, 152, 0),
                            "已结束" => Color32::from_rgb(158, 158, 158),
                            "已取消" => Color32::from_rgb(244, 67, 54),
                            _ => Color32::from_gray(150),
                        };

                        // 状态标记
                        Frame::none()
                            .fill(status_color.linear_multiply(0.2))
                            .rounding(Rounding::same(2.0))
                            .inner_margin(Vec2::new(6.0, 2.0))
                            .show(ui, |ui| {
                                ui.label(RichText::new(&project.status).color(status_color).small());
                            });

                        ui.add_space(8.0);
                        ui.strong(RichText::new(&project.name).size(16.0));
                    });

                    ui.add_space(4.0);
                    
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("📍 {}", project.city)).color(Color32::from_gray(180)).small());
                        ui.add_space(10.0);
                        ui.label(RichText::new(format!("📅 {} 场", project.screen_count)).color(Color32::from_gray(180)).small());
                        ui.add_space(10.0);
                        ui.label(RichText::new(format!("💲 ¥{}-{}", project.price_low as i32, project.price_high as i32)).color(Color32::from_gray(180)).small());
                    });
                    
                    ui.add_space(2.0);
                    
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("🕙 {}", &project.start_time[..10])).color(Color32::from_gray(160)).small());
                        ui.add_space(10.0);
                        ui.label(RichText::new(format!("🔄 更新于 {}", &project.update_time[..10])).color(Color32::from_gray(160)).small());
                    });
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("详情").clicked() {
                        clicked = true;
                    }
                });
            });
        });
        // 如果按钮被点击，返回项目ID
    if clicked {
        Some(project.id.clone())
    } else {
        None
    }
}