// Localized strings for Chinese and English.

#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Lang {
    Zh,
    En,
}

impl Lang {
    pub fn toggle(self) -> Self {
        match self {
            Lang::Zh => Lang::En,
            Lang::En => Lang::Zh,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Lang::Zh => "中",
            Lang::En => "EN",
        }
    }
}

pub struct T {
    pub lang: Lang,
}

impl T {
    pub fn new(lang: Lang) -> Self { T { lang } }

    // Deck list
    pub fn app_title(&self) -> &str { match self.lang { Lang::Zh => "扑克牌收集", Lang::En => "Poker Collection" } }
    pub fn your_decks(&self) -> &str { match self.lang { Lang::Zh => "你的牌盒", Lang::En => "Your Decks" } }
    pub fn new_deck(&self) -> &str { match self.lang { Lang::Zh => "+ 新建", Lang::En => "+ New" } }
    pub fn no_decks(&self) -> &str { match self.lang { Lang::Zh => "暂无牌盒，点击 '+ 新建' 开始收集！", Lang::En => "No decks yet. Click '+ New' to start!" } }
    pub fn delete(&self) -> &str { match self.lang { Lang::Zh => "删除", Lang::En => "Delete" } }
    pub fn new_deck_title(&self) -> &str { match self.lang { Lang::Zh => "新建牌盒", Lang::En => "New Deck" } }
    pub fn deck_name(&self) -> &str { match self.lang { Lang::Zh => "牌盒名称:", Lang::En => "Deck name:" } }
    pub fn create(&self) -> &str { match self.lang { Lang::Zh => "创建", Lang::En => "Create" } }
    pub fn cancel(&self) -> &str { match self.lang { Lang::Zh => "取消", Lang::En => "Cancel" } }

    // Card grid
    pub fn back(&self) -> &str { match self.lang { Lang::Zh => "返回", Lang::En => "Back" } }
    pub fn progress(&self) -> &str { match self.lang { Lang::Zh => "进度", Lang::En => "Progress" } }
    pub fn select_all(&self) -> &str { match self.lang { Lang::Zh => "全选", Lang::En => "Select All" } }
    pub fn deselect_all(&self) -> &str { match self.lang { Lang::Zh => "全不选", Lang::En => "Deselect All" } }
    pub fn filter(&self) -> &str { match self.lang { Lang::Zh => "筛选:", Lang::En => "Filter:" } }
    pub fn all(&self) -> &str { match self.lang { Lang::Zh => "全部", Lang::En => "All" } }
    pub fn collected(&self) -> &str { match self.lang { Lang::Zh => "已收集", Lang::En => "Collected" } }
    pub fn uncollected(&self) -> &str { match self.lang { Lang::Zh => "未收集", Lang::En => "Uncollected" } }
    pub fn rank(&self) -> &str { match self.lang { Lang::Zh => "点数:", Lang::En => "Rank:" } }
    pub fn jokers(&self) -> &str { match self.lang { Lang::Zh => "大小王", Lang::En => "Jokers" } }
    pub fn showing(&self, n: usize) -> String { match self.lang { Lang::Zh => format!("显示 {}/52 张", n), Lang::En => format!("Showing {}/52", n) } }
    pub fn collection_complete(&self) -> &str { match self.lang { Lang::Zh => "收集完成！", Lang::En => "Collection Complete!" } }
    pub fn congratulations(&self) -> &str { match self.lang { Lang::Zh => "恭喜！", Lang::En => "Congratulations!" } }
    pub fn fully_collected(&self, name: &str) -> String { match self.lang { Lang::Zh => format!("\"{}\" 已收集全！", name), Lang::En => format!("\"{}\" is fully collected!", name) } }
    pub fn ok(&self) -> &str { match self.lang { Lang::Zh => "确定", Lang::En => "OK" } }
}
