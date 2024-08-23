use chrono::NaiveDate;

//日付を"yyyy年mm月dd日"にフォーマット
pub fn format_yyyy_mm_dd(date: &NaiveDate ) -> String {
    date.format("%Y年%m月%d日").to_string()
}