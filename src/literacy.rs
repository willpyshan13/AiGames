use serde::{Deserialize, Serialize};

pub const UNITS: [&str; 3] = ["天地自然", "森林探险", "认识自己"];
pub struct Word {
    pub character: &'static str,
    pub pinyin: &'static str,
    pub meaning: &'static str,
    pub phrase: &'static str,
    pub blank: &'static str,
}
pub const WORDS: [Word; 12] = [
    Word {
        character: "日",
        pinyin: "rì",
        meaning: "太阳，也表示一天。",
        phrase: "日出",
        blank: "太阳升起来了：□出",
    },
    Word {
        character: "月",
        pinyin: "yuè",
        meaning: "月亮，也用来表示月份。",
        phrase: "月亮",
        blank: "夜空中弯弯的：□亮",
    },
    Word {
        character: "山",
        pinyin: "shān",
        meaning: "高高隆起的地面。",
        phrase: "高山",
        blank: "远处有一座高高的：高□",
    },
    Word {
        character: "水",
        pinyin: "shuǐ",
        meaning: "河里流动的水，我们每天都要喝。",
        phrase: "河水",
        blank: "小河里清清的：河□",
    },
    Word {
        character: "木",
        pinyin: "mù",
        meaning: "树木，也表示木头。",
        phrase: "木头",
        blank: "桌子可以用它做：□头",
    },
    Word {
        character: "火",
        pinyin: "huǒ",
        meaning: "燃烧时发光、发热的火焰。",
        phrase: "火苗",
        blank: "燃烧时跳动的小火焰：□苗",
    },
    Word {
        character: "土",
        pinyin: "tǔ",
        meaning: "地上的泥土，植物在里面生根。",
        phrase: "泥土",
        blank: "小种子种在：泥□",
    },
    Word {
        character: "石",
        pinyin: "shí",
        meaning: "坚硬的石头。",
        phrase: "石头",
        blank: "路边硬硬的小块：□头",
    },
    Word {
        character: "口",
        pinyin: "kǒu",
        meaning: "嘴巴，我们用它吃东西、说话。",
        phrase: "开口",
        blank: "开始说话：开□",
    },
    Word {
        character: "目",
        pinyin: "mù",
        meaning: "眼睛。",
        phrase: "目光",
        blank: "眼睛看向一个地方：□光",
    },
    Word {
        character: "手",
        pinyin: "shǒu",
        meaning: "用来拿东西、写字的身体部位。",
        phrase: "小手",
        blank: "写字、画画都要用：小□",
    },
    Word {
        character: "足",
        pinyin: "zú",
        meaning: "脚。",
        phrase: "足球",
        blank: "用脚踢的球：□球",
    },
];

#[derive(Default, Serialize, Deserialize)]
pub struct Progress {
    pub completed: [bool; 3],
    pub xp: u32,
}
pub struct Literacy {
    pub progress: Progress,
    pub unit: usize,
    pub card: usize,
    pub stage: usize,
    pub selected: Option<usize>,
    pub checked: bool,
    pub correct: bool,
    pub combo: usize,
    pub mistakes: Vec<(usize, usize)>,
    pub reviewing: bool,
    pub finished: bool,
    pub map: bool,
}
impl Default for Literacy {
    fn default() -> Self {
        Self {
            progress: Progress::default(),
            unit: 0,
            card: 0,
            stage: 0,
            selected: None,
            checked: false,
            correct: false,
            combo: 0,
            mistakes: vec![],
            reviewing: false,
            finished: false,
            map: true,
        }
    }
}
impl Literacy {
    pub fn start(&mut self, unit: usize) {
        self.unit = unit.min(2);
        self.card = 0;
        self.stage = 0;
        self.selected = None;
        self.checked = false;
        self.correct = false;
        self.combo = 0;
        self.mistakes.clear();
        self.reviewing = false;
        self.finished = false;
        self.map = false;
    }
    pub fn word_index(&self) -> usize {
        self.unit * 4 + self.card
    }
    pub fn word(&self) -> &'static Word {
        &WORDS[self.word_index()]
    }
    pub fn options(&self) -> [usize; 4] {
        let mut choices = [
            self.unit * 4,
            self.unit * 4 + 1,
            self.unit * 4 + 2,
            self.unit * 4 + 3,
        ];
        choices.rotate_left((self.card + self.stage) % 4);
        choices
    }
    /// An incorrect choice can be corrected in place, without a dead button or a forced detour.
    pub fn select(&mut self, value: usize) {
        if self.map
            || self.finished
            || self.stage == 0
            || (self.checked && self.correct)
            || !self.options().contains(&value)
        {
            return;
        }
        self.checked = false;
        self.selected = Some(value);
        self.check();
        if self.correct {
            self.mistakes
                .retain(|&(card, stage)| (card, stage) != (self.card, self.stage));
        }
    }

    pub fn check(&mut self) {
        if self.checked || self.stage == 0 || self.finished {
            return;
        }
        let Some(selected) = self.selected else {
            return;
        };
        self.correct = selected == self.word_index();
        self.checked = true;
        if self.correct {
            self.combo += 1;
        } else {
            self.combo = 0;
            if !self.mistakes.contains(&(self.card, self.stage)) {
                self.mistakes.push((self.card, self.stage));
            }
        }
    }
    pub fn next(&mut self) {
        if self.finished || (self.stage != 0 && !self.checked) {
            return;
        }
        self.selected = None;
        self.checked = false;
        if self.reviewing {
            if let Some((card, stage)) = self.mistakes.first().copied() {
                self.mistakes.remove(0);
                self.card = card;
                self.stage = stage;
            } else {
                self.finish();
            }
        } else if self.stage < 3 {
            self.stage += 1;
        } else if self.card < 3 {
            self.card += 1;
            self.stage = 0;
        } else {
            self.reviewing = true;
            if let Some((card, stage)) = self.mistakes.first().copied() {
                self.mistakes.remove(0);
                self.card = card;
                self.stage = stage;
            } else {
                self.finish();
            }
        }
    }
    fn finish(&mut self) {
        self.finished = true;
        if !self.progress.completed[self.unit] {
            self.progress.completed[self.unit] = true;
            self.progress.xp += 40;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wrong_choice_can_be_corrected_without_leaving_the_question() {
        let mut g = Literacy::default();
        g.start(0);
        g.next();
        g.select(1);
        assert!(g.checked && !g.correct);
        assert_eq!(g.mistakes, vec![(0, 1)]);
        g.select(0);
        assert!(g.correct);
        assert_eq!(g.stage, 1);
        assert!(g.mistakes.is_empty());
        assert_eq!(g.combo, 1);
        g.select(0);
        assert_eq!(g.combo, 1, "repeated taps must not award extra progress");
        g.next();
        assert_eq!(g.stage, 2);
    }

    #[test]
    fn selection_ignores_unavailable_words_and_non_question_screens() {
        let mut g = Literacy::default();
        g.select(0);
        assert!(!g.checked);
        g.start(0);
        g.select(0);
        assert!(!g.checked);
        g.next();
        g.select(11);
        assert!(!g.checked);
        assert_eq!(g.selected, None);
    }

    #[test]
    fn wrong_answers_return_in_review_and_rewards_only_once() {
        let mut g = Literacy::default();
        g.start(0);
        g.next();
        g.selected = Some(1);
        g.check();
        g.next();
        while !g.reviewing {
            if g.stage > 0 {
                g.selected = Some(g.word_index());
                g.check();
            }
            g.next();
        }
        assert_eq!((g.card, g.stage), (0, 1));
        assert!(!g.finished);
        g.selected = Some(2);
        g.check();
        g.next();
        assert!(!g.finished);
        g.selected = Some(0);
        g.check();
        g.next();
        assert!(g.finished);
        assert_eq!(g.progress.xp, 40);
        g.start(0);
        while !g.finished {
            if g.stage > 0 {
                g.selected = Some(g.word_index());
                g.check();
            }
            g.next();
        }
        assert_eq!(g.progress.xp, 40);
    }
    #[test]
    fn options_and_submission_are_valid() {
        let mut g = Literacy::default();
        for u in 0..3 {
            g.start(u);
            for c in 0..4 {
                g.card = c;
                for stage in 1..4 {
                    g.stage = stage;
                    let options = g.options();
                    assert_eq!(options.iter().filter(|&&x| x == g.word_index()).count(), 1);
                }
            }
        }
        g.start(0);
        g.next();
        g.check();
        g.next();
        assert_eq!(g.stage, 1);
    }
}
