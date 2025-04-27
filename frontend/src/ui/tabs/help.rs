use eframe::egui::{self, RichText, Color32, Stroke, Align2, Ui};
use crate::app::Myapp;

pub fn render(app: &mut Myapp, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        // 标题区域
        ui.add_space(10.0);
        ui.vertical_centered(|ui| {
            ui.heading(RichText::new("帮助与关于").size(28.0).strong());
            ui.add_space(5.0);
            ui.label(RichText::new("B站演出数据分析系统").size(16.0).italics());
        });
        ui.add_space(20.0);

        // 分栏显示帮助内容和关于信息
        ui.columns(2, |columns| {
            // 左侧：使用帮助
            render_help_section(&mut columns[0]);
            
            // 右侧：关于信息和更新日志
            render_about_section(&mut columns[1]);
        });
    });
}

fn render_help_section(ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.heading(RichText::new("使用指南").size(20.0).color(Color32::from_rgb(66, 150, 250)));
        ui.add_space(10.0);
        
        // 使用卡片式布局显示每个功能模块的说明
        help_card(ui, "数据概览", "📊", |ui| {
            ui.label("数据概览页面展示系统中所有演出项目的统计信息，包括：");
            ui.add_space(5.0);
            ui.label("• 总项目数量和新增项目统计");
            ui.label("• 最近更新的项目列表");
            ui.label("• 关键数据指标的视图展示");
            ui.add_space(5.0);
            ui.label("点击 刷新数据 按钮可获取最新数据。");
        });
        
        help_card(ui, "项目列表", "🎭", |ui| {
            ui.label("项目列表页面提供所有演出项目的详细信息，支持：");
            ui.add_space(5.0);
            ui.label("• 搜索和筛选功能");
            ui.label("• 按不同条件排序");
            ui.label("• 查看项目详情");
            ui.add_space(5.0);
            ui.label("点击项目行中的 详情 按钮可查看更多信息。");
        });
        
        help_card(ui, "数据分析", "📈", |ui| {
            ui.label("数据分析页面提供多维度的数据可视化和分析：");
            ui.add_space(5.0);
            ui.label("• 价格分布分析");
            ui.label("• 场次时间分布");
            ui.label("• 地区分布热力图");
            ui.label("• 月度趋势追踪");
            ui.add_space(5.0);
            ui.label("使用页面顶部的选择器可切换不同的分析视图。");
        });
        
        help_card(ui, "系统设置", "⚙️", |ui| {
            ui.label("系统设置页面允许自定义软件行为：");
            ui.add_space(5.0);
            ui.label("• 数据更新频率设置");
            ui.label("• 界面个性化选项");
            ui.label("• API和存储配置");
            ui.add_space(5.0);
            ui.label("修改设置后请点击 保存设置 按钮。");
        });
        
        // 常见问题解答
        ui.add_space(15.0);
        ui.heading(RichText::new("常见问题").size(20.0).color(Color32::from_rgb(66, 150, 250)));
        ui.add_space(10.0);
        
        faq_item(ui, "如何登录系统？", |ui| {
            ui.label("点击右上角用户图标，选择登录方式即可进入登录界面。系统支持多种登录方式，包括二维码登录和短信登录。");
        });
        
        faq_item(ui, "数据多久更新一次？", |ui| {
            ui.label("系统默认每30分钟自动更新一次数据。您也可以在各页面中点击 刷新数据 按钮手动更新。");
        });
        
        faq_item(ui, "如何导出分析结果？", |ui| {
            ui.label("在数据分析页面，点击图表右上角的导出按钮，可以将当前分析结果导出为图片或CSV格式。");
        });
    });
}

fn render_about_section(ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.heading(RichText::new("关于").size(20.0).color(Color32::from_rgb(66, 150, 250)));
        ui.add_space(10.0);
        
        // 关于软件
        about_card(ui, "软件信息", |ui| {
            ui.horizontal(|ui| {
                ui.strong("应用名称：");
                ui.label("B站演出数据分析系统");
            });
            ui.horizontal(|ui| {
                ui.strong("版本号：");
                ui.label("1.0.0");
            });
            ui.horizontal(|ui| {
                ui.strong("发布日期：");
                ui.label("2025年4月");
            });
            ui.horizontal(|ui| {
                ui.strong("开发语言：");
                ui.label("Rust + egui");
            });
            ui.horizontal(|ui| {
                ui.strong("运行环境：");
                ui.label("Windows/macOS/Linux");
            });
        });
        
        // 开发者信息
        about_card(ui, "开发者", |ui| {
            ui.label("本软件是基于开源社区的贡献开发而成，感谢所有参与者的付出。");
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.strong("主要开发：");
                ui.label("开发团队");
            });
            ui.horizontal(|ui| {
                ui.strong("联系方式：");
                ui.hyperlink_to("暂不显示", "hidden@example.com");
            });
            ui.horizontal(|ui| {
                ui.strong("项目主页：");
                ui.hyperlink("暂不显示");
            });
        });
        
        // 隐私声明
        about_card(ui, "隐私声明", |ui| {
            ui.label("本软件尊重并保护用户隐私，采取以下措施保障您的数据安全：");
            ui.add_space(5.0);
            ui.label("• 所有数据仅存储在本地，不会上传至第三方服务器");
            ui.label("• 账号信息经过加密存储，确保安全");
            ui.label("• 应用不会收集任何与分析目的无关的个人信息");
        });
        
        // 更新日志
        ui.add_space(15.0);
        ui.heading(RichText::new("更新日志").size(20.0).color(Color32::from_rgb(66, 150, 250)));
        ui.add_space(10.0);
        
        changelog_item(ui, "1.0.0 (2025.04)", |ui| {
            ui.label("• 初始版本发布");
            ui.label("• 实现基础数据分析功能");
            ui.label("• 支持项目列表浏览和筛选");
            ui.label("• 添加多维度数据可视化");
        });
        
        // 底部版权声明
        ui.add_space(20.0);
        ui.separator();
        ui.add_space(5.0);
        ui.vertical_centered(|ui| {
            ui.label(RichText::new("© 2025 B站演出数据分析系统").small().italics());
            ui.label(RichText::new("仅供学习和研究使用，请勿用于商业用途").small().italics());
        });
    });
}

// 帮助卡片组件
fn help_card(ui: &mut egui::Ui, title: &str, icon: &str, content: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(ui.visuals().extreme_bg_color)
        .rounding(8.0)
        .stroke(Stroke::new(1.0, Color32::from_gray(200)))
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("{} ", icon)).size(20.0));
                ui.heading(RichText::new(title).size(18.0));
            });
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);
            content(ui);
        });
    ui.add_space(10.0);
}

// 常见问题项组件
fn faq_item(ui: &mut egui::Ui, question: &str, answer: impl FnOnce(&mut egui::Ui)) {
    egui::CollapsingHeader::new(RichText::new(question).size(16.0).strong())
        .id_source(question)
        .default_open(false)
        .show(ui, |ui| {
            ui.add_space(5.0);
            answer(ui);
        });
}

// 关于卡片组件
fn about_card(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(ui.visuals().extreme_bg_color)
        .rounding(8.0)
        .stroke(Stroke::new(1.0, Color32::from_gray(200)))
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.heading(RichText::new(title).size(18.0));
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);
            content(ui);
        });
    ui.add_space(10.0);
}

// 更新日志项组件
fn changelog_item(ui: &mut egui::Ui, version: &str, changes: impl FnOnce(&mut egui::Ui)) {
    egui::CollapsingHeader::new(RichText::new(version).size(16.0).strong())
        .id_source(version)
        .default_open(true)
        .show(ui, |ui| {
            ui.add_space(5.0);
            changes(ui);
        });
}