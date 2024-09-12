#[cfg(feature = "chinese-number")]
use chinese_number::{from_chinese_to_f32_high, from_chinese_to_usize_ten_thousand};
use std::borrow::Cow;
use std::iter::Map;
use std::rc::Rc;
use std::str::SplitInclusive;

#[allow(unused)]
/// 在文字中寻找数字并返回,只查找整数。
pub fn find_number(text: &str) -> Option<usize> {
    let mut number = String::new();
    let mut found_digit = false;

    for c in text.chars() {
        if c.is_ascii_digit() {
            number.push(c);
            found_digit = true;
        } else if found_digit {
            break;
        }
    }

    if !number.is_empty() {
        number.parse::<usize>().ok()
    } else {
        None
    }
}

#[cfg(feature = "chinese-number")]
/// 查找中文数字并返回,只查找整数。
pub fn find_chinese_number(text: &str) -> Option<usize> {
    // 清除输入的空白字符
    let text = text
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    from_chinese_to_usize_ten_thousand(get_chinese_number(text.as_str()).unwrap_or_default()).ok()
}

#[cfg(feature = "chinese-number")]
#[doc(hidden)]
const CHINESE_NUMBER: &[char] = &[
    '零', '一', '二', '三', '四', '五', '六', '七', '八', '九', '十', '百', '千', '万', '亿',
];

#[cfg(feature = "chinese-number")]
#[doc(hidden)]
const CHINESE_NUMBER_TRADITIONAL: &[char] = &[
    '零', '壹', '貳', '參', '肆', '伍', '陸', '柒', '捌', '玖', '拾', '佰', '仟', '萬', '億',
];

#[cfg(feature = "chinese-number")]
pub fn get_chinese_number(text: &str) -> Option<String> {
    let mut ret = String::new();
    let mut found_digit = false;
    let chars = text.chars();
    // 判断是否是简体
    for c in chars {
        //存在于中文数字中
        let contains = CHINESE_NUMBER.contains(&c);
        //如果找到中文数字
        if contains {
            found_digit = true;
            ret.push(c);
        } else if found_digit {
            break;
        }
    }
    if found_digit {
        return Some(ret);
    }
    // 判断是否是繁体
    let chars = text.chars();
    for c in chars {
        let contains = CHINESE_NUMBER_TRADITIONAL.contains(&c);
        if contains {
            found_digit = true;
            ret.push(c);
        } else if found_digit {
            break;
        }
    }
    if found_digit {
        return Some(ret);
    }
    None
}

///一个行阅读器
// #[inline]
// pub fn lines<'a, S: AsRef<str> + 'a>(
//     s: S,
// ) -> Map<SplitInclusive<'a, char>, fn(&'a str) -> Cow<'a, str>> {
//     return s.as_ref().split_inclusive('\n').map(|line| {
//         let Some(line) = line.strip_suffix('\n') else {
//             return Cow::Owned(line.to_string());
//         };
//         let Some(line) = line.strip_suffix('\r') else {
//             return Cow::Owned(line.to_string());
//         };
//         Cow::Owned(line.to_string())
//     })
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_number() {
        let s = "第 1 章".to_string();
        let n = find_number(&s);
        assert_eq!(n, Some(1));
        let s = "第 10 章".to_string();
        let n = find_number(&s);
        assert_eq!(n, Some(10));
    }
    #[test]
    #[cfg(feature = "chinese-number")]
    fn test_find_chinese_number() {
        let s = "第 一 章".to_string();
        let n = find_chinese_number(&s);
        assert_eq!(n, Some(1));
        let s = "第 十 一 章".to_string();
        let n = find_chinese_number(&s);
        assert_eq!(n, Some(11));
    }
    #[test]
    #[cfg(feature = "chinese-number")]
    fn test_find_chinese_number_range() {
        let s = "第一章".to_string();
        let n = get_chinese_number(&s);
        assert_eq!(n, Some("一".to_string()));
        let s = "第三百六十五章".to_string();
        let n = get_chinese_number(&s);
        assert_eq!(n, Some("三百六十五".to_string()));
    }
}
