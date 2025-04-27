use eframe::egui::{self, Ui, RichText, Color32, Stroke, Align2};
use crate::app::Myapp;
use std::collections::HashMap;

pub fn render(app: &mut Myapp, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        // 标题区域 - 使用渐变背景
        ui.add_space(5.0);
        ui.vertical_centered(|ui| {
            ui.heading(RichText::new("B站演出数据分析").size(24.0).strong());
        });
        ui.add_space(16.0);
        
        // 分析控制面板 - 现代化圆角面板设计
        egui::Frame::none()
            .fill(ui.visuals().extreme_bg_color)
            .rounding(8.0)
            .stroke(Stroke::new(1.0, Color32::from_rgb(200, 200, 200)))
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.strong("分析维度: ");
                    
                    egui::ComboBox::from_id_source("analysis_dimension")
                        .selected_text(&app.selected_analysis_dimension)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut app.selected_analysis_dimension, "价格分布".to_string(), "价格分布");
                            ui.selectable_value(&mut app.selected_analysis_dimension, "场次时间分布".to_string(), "场次时间分布");
                            ui.selectable_value(&mut app.selected_analysis_dimension, "地区分布".to_string(), "地区分布");
                            ui.selectable_value(&mut app.selected_analysis_dimension, "月度趋势".to_string(), "月度趋势");
                        });
                    
                    ui.separator();
                    
                    ui.strong("时间范围: ");
                    egui::ComboBox::from_id_source("time_range")
                        .selected_text(&app.selected_time_range)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut app.selected_time_range, "最近一周".to_string(), "最近一周");
                            ui.selectable_value(&mut app.selected_time_range, "最近一月".to_string(), "最近一月");
                            ui.selectable_value(&mut app.selected_time_range, "最近三月".to_string(), "最近三月");
                            ui.selectable_value(&mut app.selected_time_range, "全部时间".to_string(), "全部时间");
                        });
                        
                    // 现代化按钮设计
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add(egui::Button::new(
                            RichText::new(" 生成分析 ")
                                .size(16.0)
                                .color(Color32::WHITE)
                            )
                            .fill(Color32::from_rgb(66, 150, 250))
                            .rounding(6.0)
                        ).clicked() {
                            app.generate_analysis_data();
                        }
                    });
                });
            });
        
        ui.add_space(20.0);
        
        // 根据选择的分析维度显示不同的图表
        match app.selected_analysis_dimension.as_str() {
            "价格分布" => render_price_distribution(app, ui),
            "场次时间分布" => render_time_distribution(app, ui),
            "地区分布" => render_region_distribution(app, ui),
            "月度趋势" => render_monthly_trend(app, ui),
            _ => { ui.label("请选择分析维度"); }
        }
    });
}

fn render_price_distribution(app: &mut Myapp, ui: &mut egui::Ui) {
    // 创建一个带阴影和圆角的现代化面板
    egui::Frame::none()
        .fill(ui.visuals().extreme_bg_color)
        .rounding(10.0)
        .shadow(egui::epaint::Shadow::small_dark())
        .inner_margin(16.0)
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("票价区间分布").size(20.0).color(Color32::from_rgb(51, 102, 204)));
            });
            ui.add_space(10.0);
            
            // 如果没有数据，显示提示信息
            if app.price_distribution.is_empty() {
                render_empty_state(ui, "暂无价格数据，请点击 生成分析 按钮");
                return;
            }
            
            // 自定义柱状图
            render_custom_bar_chart(ui, &app.price_distribution, 300.0);
            
            ui.add_space(20.0);
            
            // 数据表格 - 现代化表格设计
            render_data_table(ui, &app.price_distribution);
        });
}

fn render_time_distribution(app: &mut Myapp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("场次时间分布").size(20.0).color(Color32::from_rgb(51, 102, 204)));
    ui.add_space(10.0);
    
    // 分成两列布局
    egui::Grid::new("time_dist_layout")
        .num_columns(2)
        .spacing([20.0, 10.0])
        .show(ui, |ui| {
            // 左侧：时段分布
            ui.vertical(|ui| {
                egui::Frame::none()
                    .fill(ui.visuals().extreme_bg_color)
                    .rounding(10.0)
                    .shadow(egui::epaint::Shadow::small_dark())
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.heading("场次时段分布");
                        ui.add_space(8.0);
                        
                        // 如果没有数据，显示提示信息
                        if app.time_slots_distribution.is_empty() {
                            render_empty_state(ui, "暂无时段数据");
                            return;
                        }
                        
                        // 饼图数据和绘制
                        render_custom_pie_chart(ui, &app.time_slots_distribution, 180.0);
                    });
            });
            
            // 右侧：星期分布
            ui.vertical(|ui| {
                egui::Frame::none()
                    .fill(ui.visuals().extreme_bg_color)
                    .rounding(10.0)
                    .shadow(egui::epaint::Shadow::small_dark())
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.heading("场次星期分布");
                        ui.add_space(8.0);
                        
                        // 如果没有数据，显示提示信息
                        if app.weekday_distribution.is_empty() {
                            render_empty_state(ui, "暂无星期数据");
                            return;
                        }
                        
                        // 水平柱状图
                        render_custom_weekday_chart(ui, &app.weekday_distribution, 180.0);
                    });
            });
            
            ui.end_row();
        });
}

fn render_region_distribution(app: &mut Myapp, ui: &mut egui::Ui) {
    // 现代化面板
    egui::Frame::none()
        .fill(ui.visuals().extreme_bg_color)
        .rounding(10.0)
        .shadow(egui::epaint::Shadow::small_dark())
        .inner_margin(16.0)
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("地区分布分析").size(20.0).color(Color32::from_rgb(51, 102, 204)));
            });
            ui.add_space(10.0);
            
            // 如果没有数据，显示提示信息
            if app.region_distribution.is_empty() {
                render_empty_state(ui, "暂无地区数据，请点击 生成分析 按钮");
                return;
            }
            
            // 热力表格
            render_region_heat_table(ui, &app.region_distribution);
        });
}

fn render_monthly_trend(app: &mut Myapp, ui: &mut egui::Ui) {
    // 现代化面板
    egui::Frame::none()
        .fill(ui.visuals().extreme_bg_color)
        .rounding(10.0)
        .shadow(egui::epaint::Shadow::small_dark())
        .inner_margin(16.0)
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("月度项目趋势").size(20.0).color(Color32::from_rgb(51, 102, 204)));
            });
            ui.add_space(10.0);
            
            // 如果没有数据，显示提示信息
            if app.monthly_trend.is_empty() {
                render_empty_state(ui, "暂无月度趋势数据，请点击 生成分析 按钮");
                return;
            }
            
            // 添加调试信息
            ui.label(format!("数据点数量: {}", app.monthly_trend.len()));
            
            // 为了确保有数据，如果为空则添加测试数据
            if app.monthly_trend.is_empty() {
                ui.label("使用测试数据");
                let mut test_data = HashMap::new();
                test_data.insert(1, 10);
                test_data.insert(2, 15);
                test_data.insert(3, 25);
                test_data.insert(4, 18);
                test_data.insert(5, 30);
                render_custom_line_chart(ui, &test_data, 250.0);
            } else {
                // 折线图
                render_custom_line_chart(ui, &app.monthly_trend, 250.0);
            }
            
            ui.add_space(20.0);
            
            // 趋势分析卡片
            render_trend_analysis_card(ui, &app.monthly_trend);
        });
}

// ======== 辅助函数 ========

// 自定义现代化柱状图
fn render_custom_bar_chart(ui: &mut Ui, data: &HashMap<String, i32>, height: f32) {
    // 计算数据范围
    let max_value = data.values().copied().max().unwrap_or(1) as f32;
    
    // 分配绘图区域
    let chart_width = ui.available_width();
    let chart_height = height.max(300.0); // 确保至少300像素高
    let (response, painter) = ui.allocate_painter(
        egui::vec2(chart_width, chart_height), 
        egui::Sense::hover()
    );
    
    let rect = response.rect;
    
    // 调试测试 - 绘制一个小矩形确认绘图系统正常工作
    painter.rect_filled(
        egui::Rect::from_min_max(
            egui::pos2(rect.left() + 5.0, rect.top() + 5.0),
            egui::pos2(rect.left() + 30.0, rect.top() + 30.0)
        ),
        0.0,
        Color32::from_rgb(255, 100, 100)
    );
    
    let bar_count = data.len() as f32;
    let bar_width = (rect.width() - 80.0) / bar_count - 10.0;
    let bar_margin = 10.0;
    
    // 修复坐标轴绘制 - 使用明确的点坐标而不是向量加法
    painter.line_segment(
        [egui::pos2(rect.left() + 40.0, rect.bottom() - 20.0), 
         egui::pos2(rect.right() - 20.0, rect.bottom() - 20.0)],
        Stroke::new(1.5, Color32::from_gray(180))
    );
    
    painter.line_segment(
        [egui::pos2(rect.left() + 40.0, rect.bottom() - 20.0), 
         egui::pos2(rect.left() + 40.0, rect.top() + 20.0)],
        Stroke::new(1.5, Color32::from_gray(180))
    );
    
    // 绘制数据条
    let mut x = rect.left() + 50.0;
    for (category, value) in data {
        // 确保即使为0也有一些可见高度
        let bar_height = ((*value as f32 / max_value) * (rect.height() - 60.0)).max(2.0);
        let y_top = rect.bottom() - bar_height - 20.0;
        
        // 使用更明亮的颜色增强可见性
        painter.rect_filled(
            egui::Rect::from_min_max(
                egui::pos2(x, rect.bottom() - 20.0),
                egui::pos2(x + bar_width, y_top)
            ),
            4.0, // 使用圆角效果
            Color32::from_rgb(75, 125, 255) // 使用更亮的蓝色
        );
        
        // 绘制数值标签
        if *value > 0 {
            painter.text(
                egui::pos2(x + bar_width / 2.0, y_top - 15.0),
                Align2::CENTER_CENTER,
                value.to_string(),
                egui::FontId::proportional(14.0),
                Color32::WHITE
            );
        }
        
        // 绘制分类标签
        let truncated_label = if category.chars().count() > 8 {
            let mut truncated = String::new();
            for (i, ch) in category.chars().enumerate() {
                if i < 8 {
                    truncated.push(ch);
                } else {
                    break;
                }
            }
            format!("{}...", truncated)
        } else {
            category.clone()
        };
        
        painter.text(
            egui::pos2(x + bar_width / 2.0, rect.bottom() - 5.0),
            Align2::CENTER_CENTER,
            truncated_label,
            egui::FontId::proportional(12.0),
            ui.visuals().text_color()
        );
        
        x += bar_width + bar_margin;
    }
    
    // 添加图表边框以便更容易识别图表区域
    painter.rect_stroke(
        rect,
        0.0,
        Stroke::new(1.0, Color32::from_gray(100))
    );
}

// 自定义饼图
fn render_custom_pie_chart(ui: &mut Ui, data: &HashMap<String, i32>, size: f32) {
    let total: i32 = data.values().sum();
    let colors = vec![
        Color32::from_rgb(66, 135, 245),
        Color32::from_rgb(240, 128, 128),
        Color32::from_rgb(152, 251, 152),
        Color32::from_rgb(255, 215, 0),
    ];
    
    // 分配绘图区域
    let (response, painter) = ui.allocate_painter(
        egui::vec2(size, size), 
        egui::Sense::hover()
    );
    
    let rect = response.rect;
    let center = rect.center();
    let radius = (rect.height() / 2.0).min(rect.width() / 2.0) - 20.0;
    
    let mut start_angle = 0.0;
    let mut legend_y = rect.top() + 20.0;
    
    for (i, (slot, count)) in data.iter().enumerate() {
        let percentage = if total > 0 {
            *count as f32 / total as f32
        } else {
            0.0
        };
        
        let sweep_angle = 360.0 * percentage;
        let color = colors[i % colors.len()];
        
        // 绘制扇形
        let points = (0..=30).map(|step| {
            let t = start_angle + sweep_angle * (step as f32 / 30.0);
            let angle = t * std::f32::consts::PI / 180.0;
            center + radius * egui::vec2(angle.cos(), angle.sin())
        }).collect::<Vec<_>>();
        
        let mut path = vec![center];
        path.extend(points);
        
        painter.add(egui::Shape::Path(egui::epaint::PathShape::convex_polygon(
            path,
            color,
            Stroke::new(1.0, Color32::WHITE),
        )));
        
        // 绘制图例
        painter.rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(center.x + radius + 20.0, legend_y),
                egui::vec2(12.0, 12.0),
            ),
            2.0,
            color,
        );
        
        painter.text(
            egui::pos2(center.x + radius + 40.0, legend_y + 6.0),
            Align2::LEFT_CENTER,
            format!("{}: {}场 ({:.1}%)", slot, count, percentage * 100.0),
            egui::FontId::proportional(13.0),
            ui.visuals().text_color(),
        );
        
        legend_y += 20.0;
        start_angle += sweep_angle;
    }
}

// 自定义星期柱状图
fn render_custom_weekday_chart(ui: &mut Ui, data: &HashMap<String, i32>, height: f32) {
    let weekdays = ["周一", "周二", "周三", "周四", "周五", "周六", "周日"];
    let mut values = vec![0; 7];
    
    for (day, count) in data {
        if let Ok(idx) = day.parse::<usize>() {
            if idx > 0 && idx <= 7 {
                values[idx - 1] = *count;
            }
        }
    }
    
    // 获取最大值
    let max_value = values.iter().copied().max().unwrap_or(1) as f32;
    
    // 分配绘图区域
    let chart_width = ui.available_width();
    let (response, painter) = ui.allocate_painter(
        egui::vec2(chart_width, height), 
        egui::Sense::hover()
    );
    
    let rect = response.rect;
    let bar_height = (rect.height() - 30.0) / 7.0 - 4.0;
    
    // 绘制柱状图
    for (i, &value) in values.iter().enumerate() {
        let bar_width = (value as f32 / max_value) * (rect.width() - 80.0);
        let y = rect.top() + i as f32 * (bar_height + 4.0) + 10.0;
        
        // 绘制标签
        painter.text(
            egui::pos2(rect.left() + 25.0, y + bar_height / 2.0),
            Align2::CENTER_CENTER,
            weekdays[i],
            egui::FontId::proportional(13.0),
            ui.visuals().text_color(),
        );
        
        // 渐变色水平柱
        let color = match i {
            5 | 6 => Color32::from_rgb(255, 165, 0), // 周末用橙色
            _ => Color32::from_rgb(100, 149, 237),   // 工作日用蓝色
        };
        
        painter.rect_filled(
            egui::Rect::from_min_max(
                egui::pos2(rect.left() + 50.0, y),
                egui::pos2(rect.left() + 50.0 + bar_width, y + bar_height),
            ),
            4.0,
            color,
        );
        
        // 数值标签
        painter.text(
            egui::pos2(rect.left() + 55.0 + bar_width, y + bar_height / 2.0),
            Align2::LEFT_CENTER,
            value.to_string(),
            egui::FontId::proportional(13.0),
            ui.visuals().text_color(),
        );
    }
}

// 热力表格
fn render_region_heat_table(ui: &mut Ui, data: &HashMap<String, i32>) {
    let total: i32 = data.values().sum();
    let mut sorted_data: Vec<(String, i32)> = data.iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    
    sorted_data.sort_by(|a, b| b.1.cmp(&a.1));
    let max_value = sorted_data.first().map(|(_, v)| *v).unwrap_or(1) as f32;
    
    // 表格标题
    ui.horizontal(|ui| {
        ui.add_space(20.0);
        ui.strong(RichText::new("城市").size(16.0));
        ui.add_space(60.0);
        ui.strong(RichText::new("项目数量").size(16.0));
        ui.add_space(40.0);
        ui.strong(RichText::new("占比").size(16.0));
        ui.add_space(60.0);
        ui.strong(RichText::new("热力值").size(16.0));
    });
    
    ui.add_space(10.0);
    
    // 表格内容
    egui::ScrollArea::vertical().show(ui, |ui| {
        for (i, (region, count)) in sorted_data.iter().enumerate().take(15) {
            let percentage = if total > 0 { (*count as f32 / total as f32) * 100.0 } else { 0.0 };
            let heat_ratio = *count as f32 / max_value;
            
            // 行背景色 (隔行变色)
            let row_bg = if i % 2 == 0 {
                ui.visuals().faint_bg_color
            } else {
                ui.visuals().extreme_bg_color
            };
            
            egui::Frame::none()
                .fill(row_bg)
                .inner_margin(egui::vec2(10.0, 8.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_cross_align(egui::Align::Center), |ui| {
                            ui.add_sized(egui::vec2(80.0, ui.available_height()), egui::Label::new(region));
                        });
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_cross_align(egui::Align::Center), |ui| {
                            ui.add_sized(egui::vec2(100.0, ui.available_height()), egui::Label::new(count.to_string()));
                        });
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_cross_align(egui::Align::Center), |ui| {
                            ui.add_sized(egui::vec2(80.0, ui.available_height()), egui::Label::new(format!("{:.1}%", percentage)));
                        });
                        
                        // 热力条
                        let bar_width = 150.0 * heat_ratio;
                        let (response, painter) = ui.allocate_painter(
                            egui::vec2(150.0, 16.0), 
                            egui::Sense::hover()
                        );
                        
                        painter.rect_filled(
                            egui::Rect::from_min_max(
                                response.rect.left_top(),
                                egui::pos2(response.rect.left() + bar_width, response.rect.bottom())
                            ),
                            4.0,
                            Color32::from_rgb(
                                (255.0 * heat_ratio) as u8, 
                                (100.0 * (1.0 - heat_ratio)) as u8, 
                                50,
                            ),
                        );
                    });
                });
        }
    });
}

// 折线图
fn render_custom_line_chart(ui: &mut Ui, data: &HashMap<i32, i32>, height: f32) {
    // 获取数据范围
    let mut months: Vec<i32> = data.keys().copied().collect();
    months.sort(); // 确保月份排序
    
    if months.is_empty() {
        ui.label("没有足够的数据绘制折线图");
        return;
    }
    
    let min_month = *months.first().unwrap();
    let max_month = *months.last().unwrap();
    
    let values: Vec<i32> = data.values().copied().collect();
    let min_value = *values.iter().min().unwrap_or(&0);
    let max_value = (*values.iter().max().unwrap_or(&100)).max(min_value + 1); // 保证最小有1的差值
    let value_range = (max_value - min_value) as f32;
    
    // 添加调试信息
    ui.label(format!("数据点: {} 个, 月份范围: {}-{}, 值范围: {}-{}", 
                     data.len(), min_month, max_month, min_value, max_value));
    
    // 分配绘图区域
    let (response, painter) = ui.allocate_painter(
        egui::vec2(ui.available_width(), height), 
        egui::Sense::hover()
    );
    
    let rect = response.rect;
    
    // 调试矩形 - 确认绘图系统工作正常
    painter.rect_filled(
        egui::Rect::from_min_max(
            egui::pos2(rect.left() + 5.0, rect.top() + 5.0),
            egui::pos2(rect.left() + 30.0, rect.top() + 30.0)
        ),
        0.0,
        Color32::from_rgb(255, 100, 100)
    );
    
    // 使用更明确的内部矩形计算
    let inner_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 40.0, rect.top() + 20.0),
        egui::pos2(rect.right() - 20.0, rect.bottom() - 30.0)
    );
    
    let chart_width = inner_rect.width();
    let chart_height = inner_rect.height();
    
    // 增加绘图区域边框
    painter.rect_stroke(
        inner_rect,
        0.0,
        Stroke::new(1.0, Color32::from_gray(100))
    );
    
    // 绘制坐标轴 - 使用明确的坐标点
    painter.line_segment(
        [
            egui::pos2(inner_rect.left(), inner_rect.bottom()),
            egui::pos2(inner_rect.right(), inner_rect.bottom())
        ],
        Stroke::new(1.5, Color32::from_gray(150))
    );
    
    painter.line_segment(
        [
            egui::pos2(inner_rect.left(), inner_rect.bottom()),
            egui::pos2(inner_rect.left(), inner_rect.top())
        ],
        Stroke::new(1.5, Color32::from_gray(150))
    );
    
    // 坐标轴刻度
    for i in 0..=4 {
        let y = inner_rect.bottom() - (i as f32 * chart_height / 4.0);
        let value = min_value + (i as f32 * value_range / 4.0) as i32;
        
        painter.line_segment(
            [
                egui::pos2(inner_rect.left(), y), 
                egui::pos2(inner_rect.left() - 5.0, y)
            ],
            Stroke::new(1.0, Color32::from_gray(150))
        );
        
        painter.text(
            egui::pos2(inner_rect.left() - 10.0, y),
            Align2::RIGHT_CENTER,
            value.to_string(),
            egui::FontId::proportional(12.0),
            ui.visuals().text_color()
        );
    }
    
    // 月份刻度
    for &month in &months {
        let x_ratio = if max_month != min_month {
            (month - min_month) as f32 / (max_month - min_month) as f32
        } else {
            0.5 // 只有一个月的情况
        };
        
        let x = inner_rect.left() + x_ratio * chart_width;
        
        painter.line_segment(
            [
                egui::pos2(x, inner_rect.bottom()), 
                egui::pos2(x, inner_rect.bottom() + 5.0)
            ],
            Stroke::new(1.0, Color32::from_gray(150))
        );
        
        painter.text(
            egui::pos2(x, inner_rect.bottom() + 15.0),
            Align2::CENTER_CENTER,
            format!("{}月", month),
            egui::FontId::proportional(12.0),
            ui.visuals().text_color()
        );
    }
    
    // 收集并排序数据点
    let mut points = Vec::new();
    for &month in &months {
        if let Some(&value) = data.get(&month) {
            let x_ratio = if max_month != min_month {
                (month - min_month) as f32 / (max_month - min_month) as f32
            } else {
                0.5 // 只有一个月的情况
            };
            
            let x = inner_rect.left() + x_ratio * chart_width;
            let y = if value_range > 0.0 {
                inner_rect.bottom() - ((value - min_value) as f32 / value_range) * chart_height
            } else {
                inner_rect.bottom() - chart_height / 2.0 // 没有范围时居中
            };
            
            points.push((x, y));
        }
    }
    
    // 确保点按X坐标排序
    points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    
    // 绘制连接线 - 修改为更粗、更明亮的线条
    if points.len() >= 2 {
        for i in 0..points.len() - 1 {
            let (x1, y1) = points[i];
            let (x2, y2) = points[i + 1];
            
            // 使用更明亮的颜色和更粗的线
            painter.line_segment(
                [egui::pos2(x1, y1), egui::pos2(x2, y2)],
                Stroke::new(3.0, Color32::from_rgb(100, 180, 255)) // 更亮更粗
            );
        }
    }
    
    // 绘制数据点
    for &(x, y) in &points {
        // 绘制外圈
        painter.circle_filled(
            egui::pos2(x, y),
            6.0,
            Color32::WHITE
        );
        
        // 绘制内圈
        painter.circle_filled(
            egui::pos2(x, y),
            4.0,
            Color32::from_rgb(66, 135, 245)
        );
    }
    
    // 添加数据值标签
    for (i, &month) in months.iter().enumerate() {
        if let Some(&value) = data.get(&month) {
            let (x, y) = points[i];
            painter.text(
                egui::pos2(x, y - 15.0),
                Align2::CENTER_CENTER,
                value.to_string(),
                egui::FontId::proportional(13.0),
                Color32::WHITE
            );
        }
    }
}

// 趋势分析卡片
fn render_trend_analysis_card(ui: &mut Ui, data: &HashMap<i32, i32>) {
    // 趋势卡片
    egui::Frame::none()
        .fill(ui.visuals().faint_bg_color)
        .rounding(8.0)
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("趋势分析");
            });
            
            ui.add_space(8.0);
            
            if let (Some(min_value), Some(max_value)) = (
                data.values().min(), 
                data.values().max()
            ) {
                let growth_rate = if *min_value > 0 {
                    (*max_value as f32 - *min_value as f32) / *min_value as f32 * 100.0
                } else {
                    0.0
                };
                
                // 数据卡片样式
                egui::Grid::new("trend_analysis_grid")
                    .num_columns(2)
                    .spacing([20.0, 12.0])
                    .show(ui, |ui| {
                        ui.strong("数据范围:");
                        ui.label(RichText::new(format!("{} - {}", min_value, max_value)).monospace());
                        ui.end_row();
                        
                        ui.strong("最大增长率:");
                        let color = if growth_rate > 0.0 { Color32::GREEN } else { Color32::RED };
                        ui.label(RichText::new(format!("{:.1}%", growth_rate)).color(color).monospace());
                        ui.end_row();
                        
                        // 计算趋势方向
                        let trend_direction = if data.len() >= 3 {
                            let values: Vec<(i32, i32)> = data.iter()
                                .map(|(&k, &v)| (k, v))
                                .collect();
                            let sorted_values = {
                                let mut v = values.clone();
                                v.sort_by_key(|&(k, _)| k);
                                v
                            };
                            
                            if sorted_values.len() >= 3 {
                                let last_three = &sorted_values[sorted_values.len() - 3..];
                                if last_three[2].1 > last_three[0].1 {
                                    "上升"
                                } else if last_three[2].1 < last_three[0].1 {
                                    "下降"
                                } else {
                                    "稳定"
                                }
                            } else {
                                "数据不足"
                            }
                        } else {
                            "数据不足"
                        };
                        
                        ui.strong("趋势方向:");
                        let color = match trend_direction {
                            "上升" => Color32::GREEN,
                            "下降" => Color32::RED,
                            _ => Color32::YELLOW,
                        };
                        ui.label(RichText::new(trend_direction).color(color).monospace().strong());
                        ui.end_row();
                    });
            } else {
                ui.label("数据不足，无法分析趋势");
            }
        });
}

// 空状态显示
fn render_empty_state(ui: &mut Ui, message: &str) {
    let available_size = ui.available_size();
    
    ui.allocate_ui_at_rect(
        egui::Rect::from_center_size(
            ui.min_rect().center(),
            egui::vec2(available_size.x, 100.0)
        ),
        |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.add(egui::widgets::Spinner::new().size(24.0));
                ui.add_space(10.0);
                ui.label(RichText::new(message).size(16.0).color(Color32::from_gray(180)));
            });
        }
    );
}

// 数据表格
fn render_data_table(ui: &mut Ui, data: &HashMap<String, i32>) {
    let total: i32 = data.values().sum();
    
    egui::Frame::none()
        .fill(ui.visuals().faint_bg_color)
        .rounding(6.0)
        .show(ui, |ui| {
            egui::Grid::new("data_table")
                .striped(true)
                .num_columns(3)
                .min_col_width(120.0)
                .spacing([12.0, 8.0])
                .show(ui, |ui| {
                    // 表头
                    ui.strong(RichText::new("价格区间").size(15.0));
                    ui.strong(RichText::new("项目数量").size(15.0));
                    ui.strong(RichText::new("占比").size(15.0));
                    ui.end_row();
                    
                    // 行数据
                    for (price_range, count) in data {
                        ui.label(price_range);
                        ui.label(count.to_string());
                        let percentage = if total > 0 {
                            (*count as f32 / total as f32) * 100.0
                        } else {
                            0.0
                        };
                        ui.label(format!("{:.1}%", percentage));
                        ui.end_row();
                    }
                    
                    // 总计行
                    ui.separator();
                    ui.strong("总计");
                    ui.strong(total.to_string());
                    ui.strong("100%");
                    ui.end_row();
                });
        });
}