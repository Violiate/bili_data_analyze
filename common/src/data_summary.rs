use std::collections::HashMap;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rand::seq::SliceRandom;
use chrono::{Local, Duration, NaiveDateTime, NaiveDate};

// 项目概要数据模型
#[derive(Clone, Debug)]
pub struct ProjectSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub city: String,
    pub venue: String,
    pub price_low: f32,
    pub price_high: f32,
    pub start_time: String,
    pub end_time: String,
    pub update_time: String,
    pub status: String,
    pub screen_count: i32,
    pub image_url: Option<String>,
}

// 场馆信息
#[derive(Clone, Debug)]
pub struct VenueInfo {
    pub name: String,
    pub city: String,
    pub address: String,
}

// 场次信息
#[derive(Clone, Debug)]
pub struct ScreenInfo {
    pub id: String,
    pub start_time: String,
    pub end_time: String,
    pub ticket_status: String,
    pub venue_name: String,
}

/// 生成假数据的工具结构
pub struct MockDataGenerator;

impl MockDataGenerator {
    // 生成随机ID
    fn generate_id() -> String {
        let mut rng = thread_rng();
        std::iter::repeat_with(|| rng.sample(Alphanumeric))
            .map(char::from)
            .take(12)
            .collect()
    }
    
    // 生成随机演出名称
    fn generate_project_name() -> String {
        let prefixes = [
            "「梦境之旅」", "「星光璀璨」", "「流行盛典」", "「经典回归」",
            "「潮流之夜」", "「音乐盛宴」", "「闪耀舞台」", "「动漫音乐会」",
            "「摇滚现场」", "「民谣之声」", "「电音派对」", "「爵士夜话」",
            "「交响乐章」", "「情歌对唱」", "「说唱竞技场」", "「百老汇经典」"
        ];
        
        let performers = [
            "周杰伦", "林俊杰", "张惠妹", "五月天",
            "薛之谦", "陈奕迅", "王力宏", "李荣浩",
            "蔡依林", "华晨宇", "王嘉尔", "毛不易",
            "许嵩", "刘德华", "张学友", "邓紫棋"
        ];
        
        let suffixes = [
            "演唱会", "音乐会", "巡回演出", "粉丝见面会",
            "专场演出", "新专辑首唱会", "交响音乐会", "线上直播",
            "跨年演唱会", "生日会", "主题演唱会", "首唱会"
        ];
        
        let mut rng = thread_rng();
        let prefix = prefixes.choose(&mut rng).unwrap();
        let performer = performers.choose(&mut rng).unwrap();
        let suffix = suffixes.choose(&mut rng).unwrap();
        
        format!("{}{}{}", prefix, performer, suffix)
    }
    
    // 生成随机描述
    fn generate_description() -> String {
        let descriptions = [
            "一场视听盛宴，带你体验不一样的音乐世界，感受音乐的无限魅力。",
            "超高人气偶像倾情演出，粉丝互动环节丰富，不容错过的精彩现场。",
            "经典歌曲现场演绎，原汁原味的音乐体验，重温经典的难忘瞬间。",
            "全新专辑曲目首次公开，独家现场体验，与偶像零距离接触的机会。",
            "集结多位明星嘉宾，精彩表演轮番上阵，视听感受前所未有。",
            "首次世界巡演中国站，国际级舞美设计，顶尖音响设备打造极致体验。",
            "跨界合作音乐会，不同风格的碰撞，带来全新的听觉感受。",
            "将流行元素与古典音乐完美融合，打破传统演出模式的创新尝试。"
        ];
        
        let mut rng = thread_rng();
        descriptions.choose(&mut rng).unwrap().to_string()
    }
    
    // 生成随机城市
    fn generate_city() -> String {
        let cities = [
            "北京", "上海", "广州", "深圳", "成都",
            "杭州", "南京", "武汉", "西安", "重庆",
            "苏州", "长沙", "郑州", "天津", "青岛",
            "福州", "厦门", "沈阳", "大连", "哈尔滨"
        ];
        
        let mut rng = thread_rng();
        cities.choose(&mut rng).unwrap().to_string()
    }
    
    // 生成随机场馆
    fn generate_venue(city: &str) -> String {
        let venues = match city {
            "北京" => vec!["国家体育场", "工人体育场", "五棵松体育馆", "北京展览馆剧场"],
            "上海" => vec!["梅赛德斯奔驰文化中心", "上海体育馆", "上海大舞台", "万代南梦宫上海文化中心"],
            "广州" => vec!["广州体育馆", "广州大剧院", "中山纪念堂", "广州天河体育中心"],
            _ => vec!["体育中心", "大剧院", "演艺中心", "文化艺术中心", "体育馆", "音乐厅"],
        };
        
        let mut rng = thread_rng();
        venues.choose(&mut rng).unwrap().to_string()
    }
    
    // 生成随机价格范围
    fn generate_price_range() -> (f32, f32) {
        let price_ranges = [
            (80.0, 180.0),
            (120.0, 380.0),
            (180.0, 480.0),
            (280.0, 680.0),
            (380.0, 980.0),
            (580.0, 1280.0),
            (880.0, 1680.0),
            (1280.0, 2680.0),
        ];
        
        let mut rng = thread_rng();
        *price_ranges.choose(&mut rng).unwrap()
    }
    
    // 生成随机日期时间
    fn generate_datetime(from_days: i64, to_days: i64) -> String {
        let mut rng = thread_rng();
        let days = rng.gen_range(from_days..=to_days);
        let hours = rng.gen_range(0..23);
        
        let date = Local::now()
            .checked_add_signed(Duration::days(days))
            .unwrap();
            
        let naive_time = NaiveDateTime::new(
            date.date_naive(),
            chrono::NaiveTime::from_hms_opt(hours, 0, 0).unwrap()
        );
        
        naive_time.format("%Y-%m-%d %H:%M:%S").to_string()
    }
    
    // 生成随机状态
    fn generate_status() -> String {
        let statuses = ["售票中", "即将开售", "已售罄", "已结束", "已取消"];
        let weights = [60, 20, 10, 8, 2]; // 权重分配，让"售票中"的概率更高
        
        let mut rng = thread_rng();
        let mut cumulative_weight = 0;
        let total_weight: i32 = weights.iter().sum();
        let random_value = rng.gen_range(0..total_weight);
        
        for (i, &weight) in weights.iter().enumerate() {
            cumulative_weight += weight;
            if random_value < cumulative_weight {
                return statuses[i].to_string();
            }
        }
        
        statuses[0].to_string() // 默认返回"售票中"
    }
    
    // 生成随机场次数量
    fn generate_screen_count() -> i32 {
        let mut rng = thread_rng();
        rng.gen_range(1..=12)
    }
    
    // 生成随机项目
    pub fn generate_random_project() -> ProjectSummary {
        let id = Self::generate_id();
        let name = Self::generate_project_name();
        let description = Self::generate_description();
        let city = Self::generate_city();
        let venue = Self::generate_venue(&city);
        let (price_low, price_high) = Self::generate_price_range();
        let start_time = Self::generate_datetime(7, 120); // 7-120天后
        let end_time = {
            let naive_dt = NaiveDateTime::parse_from_str(&start_time, "%Y-%m-%d %H:%M:%S").unwrap();
            let hours_to_add = thread_rng().gen_range(2..4);
            let end_dt = naive_dt + Duration::hours(hours_to_add);
            end_dt.format("%Y-%m-%d %H:%M:%S").to_string()
        };
        let update_time = Self::generate_datetime(-30, 0); // 0-30天前
        let status = Self::generate_status();
        let screen_count = Self::generate_screen_count();
        
        ProjectSummary {
            id,
            name,
            description,
            city,
            venue,
            price_low,
            price_high,
            start_time,
            end_time,
            update_time,
            status,
            screen_count,
            image_url: None,
        }
    }
    
    // 生成多个随机项目
    pub fn generate_projects(count: usize) -> Vec<ProjectSummary> {
        (0..count).map(|_| Self::generate_random_project()).collect()
    }
    
    // 生成价格分布数据
    pub fn generate_price_distribution() -> HashMap<String, i32> {
        let mut distribution = HashMap::new();
        
        distribution.insert("0-100元".to_string(), 15);
        distribution.insert("100-200元".to_string(), 28);
        distribution.insert("200-500元".to_string(), 42);
        distribution.insert("500-1000元".to_string(), 18);
        distribution.insert("1000元以上".to_string(), 7);
        
        distribution
    }
    
    // 生成时段分布数据
    pub fn generate_time_slots_distribution() -> HashMap<String, i32> {
        let mut distribution = HashMap::new();
        
        distribution.insert("上午场".to_string(), 12);
        distribution.insert("下午场".to_string(), 38);
        distribution.insert("晚上场".to_string(), 56);
        distribution.insert("深夜场".to_string(), 4);
        
        distribution
    }
    
    // 生成星期分布数据
    pub fn generate_weekday_distribution() -> HashMap<String, i32> {
        let mut distribution = HashMap::new();
        
        distribution.insert("1".to_string(), 15); // 周一
        distribution.insert("2".to_string(), 18); // 周二
        distribution.insert("3".to_string(), 12); // 周三
        distribution.insert("4".to_string(), 20); // 周四
        distribution.insert("5".to_string(), 25); // 周五
        distribution.insert("6".to_string(), 42); // 周六
        distribution.insert("7".to_string(), 38); // 周日
        
        distribution
    }
    
    // 生成地区分布数据
    pub fn generate_region_distribution() -> HashMap<String, i32> {
        let mut distribution = HashMap::new();
        
        distribution.insert("北京".to_string(), 42);
        distribution.insert("上海".to_string(), 38);
        distribution.insert("广州".to_string(), 25);
        distribution.insert("深圳".to_string(), 22);
        distribution.insert("成都".to_string(), 18);
        distribution.insert("杭州".to_string(), 15);
        distribution.insert("武汉".to_string(), 12);
        distribution.insert("南京".to_string(), 10);
        distribution.insert("西安".to_string(), 8);
        distribution.insert("重庆".to_string(), 7);
        distribution.insert("苏州".to_string(), 6);
        distribution.insert("长沙".to_string(), 6);
        distribution.insert("郑州".to_string(), 5);
        distribution.insert("天津".to_string(), 5);
        distribution.insert("青岛".to_string(), 4);
        
        distribution
    }
    
    // 生成月度趋势数据
    pub fn generate_monthly_trend() -> HashMap<i32, i32> {
        let mut trend = HashMap::new();
        
        trend.insert(1, 23);  // 1月
        trend.insert(2, 28);  // 2月
        trend.insert(3, 32);  // 3月
        trend.insert(4, 38);  // 4月
        trend.insert(5, 45);  // 5月
        trend.insert(6, 52);  // 6月
        trend.insert(7, 48);  // 7月
        trend.insert(8, 42);  // 8月
        trend.insert(9, 50);  // 9月
        trend.insert(10, 58); // 10月
        trend.insert(11, 63); // 11月
        trend.insert(12, 70); // 12月
        
        trend
    }
}