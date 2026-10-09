pub const LEVELS: [&str; 5] = ["入门", "基础", "提高", "挑战", "竞赛"];

pub struct Question {
    pub prompt: &'static str,
    pub answer: i32,
    pub hint: &'static str,
    pub explanation: &'static str,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MathGame {
    pub placed: i32,
    pub choosing_level: bool,
    pub level: usize,
    pub index: usize,
    pub stars: [usize; 5],
    #[serde(default)]
    pub correct_total: u32,
    pub solved: bool,
    pub hinted: bool,
    pub wrong: Option<i32>,
}

impl Default for MathGame {
    fn default() -> Self {
        Self {
            placed: 0,
            choosing_level: false,
            level: 0,
            index: 0,
            stars: [0; 5],
            correct_total: 0,
            solved: false,
            hinted: false,
            wrong: None,
        }
    }
}
impl MathGame {
    pub fn question(&self) -> Question {
        question(self.level, self.index)
    }
    pub fn choose_level(&mut self, level: usize) {
        self.placed = 0;
        self.choosing_level = false;
        self.level = level;
        self.index = self.stars[level].min(7);
        self.solved = false;
        self.hinted = false;
        self.wrong = None;
    }
    pub fn answer(&mut self, value: i32) {
        if self.solved {
            return;
        }
        if value == self.question().answer {
            self.solved = true;
            self.correct_total = self.correct_total.saturating_add(1);
            self.stars[self.level] = self.stars[self.level].max(self.index + 1);
            self.wrong = None;
        } else {
            self.wrong = Some(value);
        }
    }
    pub fn next(&mut self) {
        if !self.solved {
            return;
        }
        self.placed = 0;
        if self.index < 7 {
            self.index += 1;
        } else {
            self.index = 0;
        }
        self.solved = false;
        self.hinted = false;
        self.wrong = None;
    }
    pub fn choices(&self) -> [i32; 4] {
        let a = self.question().answer;
        let mut values = [a, a + 1, a + 3, if a > 1 { a - 1 } else { a + 2 }];
        values.rotate_left((self.level + self.index) % 4);
        values
    }
    pub fn place(&mut self, delta: i32) {
        if !self.solved {
            self.placed = (self.placed + delta).clamp(0, 99);
            self.wrong = None;
        }
    }
}

pub fn question(level: usize, index: usize) -> Question {
    let bank = match level {
        0 => [
            (
                "小兔有 3 根胡萝卜，又找到 2 根。\n现在一共有几根？",
                5,
                "把 3 根和 2 根合起来数。",
                "从 3 接着数：4、5。3 + 2 = 5。",
            ),
            (
                "5 只小鸟飞走了 1 只。\n树上还剩几只？",
                4,
                "飞走了，就是从原来的数量里减掉。",
                "5 - 1 = 4，树上还剩 4 只。",
            ),
            (
                "1、2、3、□、5\n空格里应该填几？",
                4,
                "这些数一个比一个多 1。",
                "3 的后面是 4，4 的后面是 5。",
            ),
            (
                "比 6 多 1 的数是多少？",
                7,
                "从 6 往后数一步。",
                "6 + 1 = 7。",
            ),
            (
                "4 颗红星和 4 颗蓝星，\n一共有几颗星？",
                8,
                "两组星星合起来。",
                "4 + 4 = 8。",
            ),
            (
                "9 块饼干吃掉 3 块，\n还剩几块？",
                6,
                "从 9 开始往回数 3 次。",
                "9 - 3 = 6。",
            ),
            (
                "小熊从左边数排第 3，\n它前面有几只小动物？",
                2,
                "第 3 个的前面是第 1 个和第 2 个。",
                "排第 3，前面有 3 - 1 = 2 只。",
            ),
            (
                "盒子里有 7 颗糖，\n再放几颗就有 10 颗？",
                3,
                "从 7 数到 10，还要数几下？",
                "7 + 3 = 10，还要放 3 颗。",
            ),
        ],
        1 => [
            (
                "8 + 5 = ？",
                13,
                "先从 5 里拿 2，和 8 凑成 10。",
                "8 + 2 + 3 = 10 + 3 = 13。",
            ),
            (
                "14 - 6 = ？",
                8,
                "可以先减 4 到 10，再减 2。",
                "14 - 4 - 2 = 10 - 2 = 8。",
            ),
            ("9 + 7 = ？", 16, "把 7 分成 1 和 6。", "9 + 1 + 6 = 16。"),
            (
                "17 - 9 = ？",
                8,
                "想一想，9 加几等于 17？",
                "9 + 8 = 17，所以 17 - 9 = 8。",
            ),
            (
                "3 个十和 6 个一合起来是多少？",
                36,
                "3 个十是 30。",
                "30 + 6 = 36。",
            ),
            (
                "比 45 多 10 的数是多少？",
                55,
                "多一个十，个位不变。",
                "45 + 10 = 55。",
            ),
            (
                "小明有 12 张贴纸，小红有 8 张。\n小明比小红多几张？",
                4,
                "比较相差多少，可以用减法。",
                "12 - 8 = 4，小明多 4 张。",
            ),
            (
                "□ + 7 = 15\n空格里填几？",
                8,
                "从总数 15 中去掉已知的 7。",
                "15 - 7 = 8，所以空格填 8。",
            ),
        ],
        2 => [
            (
                "公交车上有 16 人，下去 7 人，\n又上来 5 人。现在有几人？",
                14,
                "按先下车、再上车的顺序算。",
                "16 - 7 = 9，9 + 5 = 14。",
            ),
            (
                "一本书有 30 页，已经读了 12 页。\n还剩多少页没读？",
                18,
                "总页数减去已读的页数。",
                "30 - 12 = 18。",
            ),
            (
                "买铅笔花 6 元，买本子花 8 元。\n付了 20 元，应找回几元？",
                6,
                "先算一共花了多少钱。",
                "6 + 8 = 14，20 - 14 = 6。",
            ),
            (
                "小兔有 24 根胡萝卜，\n小熊比它少 7 根。小熊有几根？",
                17,
                "少 7 根就是减去 7。",
                "24 - 7 = 17。",
            ),
            (
                "一排 15 个小朋友，小丽排第 6。\n她后面有几人？",
                9,
                "减去前 6 人，其中包含小丽。",
                "15 - 6 = 9，她后面有 9 人。",
            ),
            (
                "现在是 3 时，再过 2 小时是几时？",
                5,
                "时针从 3 向前走两大格。",
                "3 + 2 = 5，所以是 5 时。",
            ),
            ("28 + 6 = ？", 34, "先加 2 凑到 30。", "28 + 2 + 4 = 34。"),
            ("42 - 8 = ？", 34, "先减 2 到 40。", "42 - 2 - 6 = 34。"),
        ],
        3 => [
            (
                "2、4、6、8、□\n下一个数是多少？",
                10,
                "比较每两个相邻数的差。",
                "每次加 2，8 + 2 = 10。",
            ),
            (
                "1、3、6、10、□\n下一个数是多少？",
                15,
                "每次增加的数是 2、3、4……",
                "接下来加 5，10 + 5 = 15。",
            ),
            (
                "小兔从前数排第 4，从后数排第 5。\n这队一共有几只动物？",
                8,
                "两次都数到了小兔，要去掉一次。",
                "4 + 5 - 1 = 8。",
            ),
            (
                "两篮苹果共 12 个，第一篮有 7 个。\n第一篮比第二篮多几个？",
                2,
                "先求第二篮有多少个。",
                "12 - 7 = 5，7 - 5 = 2。",
            ),
            (
                "红、蓝、黄、红、蓝、黄……\n前 9 个球中，有几个红球？",
                3,
                "每 3 个球是一组，每组一个红球。",
                "前 9 个球有 3 组，所以有 3 个红球。",
            ),
            (
                "用 1、2、3 三张卡片，\n能组成几个没有重复数字的两位数？",
                6,
                "按十位是 1、2、3 分别列出来。",
                "12、13、21、23、31、32，共 6 个。",
            ),
            (
                "小明今年 7 岁，姐姐今年 12 岁。\n再过 3 年，姐姐比小明大几岁？",
                5,
                "两个人都会长大同样多的岁数。",
                "年龄差不变：12 - 7 = 5 岁。",
            ),
            (
                "一个数加 6，再减 4，结果是 13。\n这个数原来是多少？",
                11,
                "倒着想：先把减去的 4 加回来。",
                "13 + 4 - 6 = 11。",
            ),
        ],
        _ => [
            (
                "1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9\n怎样巧算？结果是多少？",
                45,
                "把首尾配对：1 和 9，2 和 8……",
                "四组各得 10，中间还有 5：40 + 5 = 45。",
            ),
            (
                "小明给小红 2 颗糖后，两人一样多。\n原来小明比小红多几颗？",
                4,
                "小明少了 2 颗，小红又多了 2 颗。",
                "两边的差共缩小 2 + 2 = 4 颗。",
            ),
            (
                "5 棵树排成一行，相邻两棵间隔 2 米。\n第一棵到最后一棵相距几米？",
                8,
                "5 棵树之间有几个空隙？",
                "有 4 个空隙，2 + 2 + 2 + 2 = 8 米。",
            ),
            (
                "鸡和兔共 3 只，一共有 8 条腿。\n其中有几只兔？",
                1,
                "先假设全部是鸡，会有几条腿？",
                "3 只鸡有 6 条腿，多出 2 条，换 1 只兔即可。",
            ),
            (
                "一只蜗牛要爬到 6 米高处。\n白天爬 3 米，夜里退 2 米，几天到顶？",
                4,
                "到顶当天就不用再往下退了。",
                "前三天夜晚后分别在 1、2、3 米，第 4 天爬到 6 米。",
            ),
            (
                "3 个小朋友，每两人握一次手。\n一共握几次手？",
                3,
                "把他们叫作甲、乙、丙，列出每一对。",
                "甲乙、甲丙、乙丙，共 3 次，不能重复数。",
            ),
            (
                "两个相同的数相加等于 14。\n其中一个数加 5 等于多少？",
                12,
                "先找出两个相同的数。",
                "7 + 7 = 14，所以 7 + 5 = 12。",
            ),
            (
                "袋里有红球和蓝球，数量都足够。\n闭眼至少摸几个，才能保证有两个同色？",
                3,
                "最不巧时，前两个一红一蓝。",
                "第 3 个不论红蓝，都与前面一个同色，所以是 3。",
            ),
        ],
    };
    let (prompt, answer, hint, explanation) = bank[index % 8];
    Question {
        prompt,
        answer,
        hint,
        explanation,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn physical_answer_retries_and_resets_between_questions() {
        let mut game = MathGame::default();
        game.place(-1);
        assert_eq!(game.placed, 0);
        game.place(4);
        game.answer(game.placed);
        assert!(!game.solved);
        game.place(1);
        game.answer(game.placed);
        assert!(game.solved);
        game.place(1);
        assert_eq!(game.placed, 5);
        game.next();
        assert_eq!(game.placed, 0);
        assert_eq!(game.index, 1);
    }
    #[test]
    fn all_questions_have_unique_choices_and_one_correct_answer() {
        let mut game = MathGame::default();
        for level in 0..5 {
            for index in 0..8 {
                game.level = level;
                game.index = index;
                let choices = game.choices();
                assert_eq!(
                    choices
                        .iter()
                        .filter(|&&x| x == game.question().answer)
                        .count(),
                    1
                );
                for i in 0..4 {
                    assert!(choices[i] >= 0);
                    for j in i + 1..4 {
                        assert_ne!(choices[i], choices[j]);
                    }
                }
            }
        }
    }
    #[test]
    fn wrong_answers_retry_and_rewards_cannot_be_farmed() {
        let mut game = MathGame::default();
        game.answer(-1);
        game.next();
        assert_eq!(game.index, 0);
        game.answer(5);
        game.answer(5);
        assert_eq!(game.stars[0], 1);
        for level in 0..5 {
            game.choose_level(level);
            game.index = 0;
            for _ in 0..8 {
                game.answer(game.question().answer);
                game.next();
            }
            assert_eq!(game.level, level, "difficulty changes only when chosen");
        }
        assert_eq!(game.stars, [8; 5]);
        assert_eq!(game.index, 0);
        assert_eq!(game.level, 4);
    }
}
