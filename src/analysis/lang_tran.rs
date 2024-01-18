// use std::cell::OnceCell;
use maplit::hashmap;
use once_cell::sync::OnceCell;
use std::collections::HashMap;

//tem_* means the identifier used in template, avoid confused with variable in function.
//tem_* should decide by language ONLY, should NOT be dynamic.
pub static LANG_MAP: OnceCell<H> = OnceCell::new();

type H = HashMap<&'static str, HashMap<&'static str, &'static str>>;

pub fn init_lang_map() {
    let e = hashmap! {
        "tem_language" => "en-US",
        "tem_day" => "DAY",
        "tem_night" => "NIGHT",
        "tem_previous_day" => "previous day",
        "tem_previous_shift" => "previous shift",
        "tem_today" => "today",
        "tem_viewing_data" => "Viewing data",
        "tem_colon" => ": ",
        "tem_home" => "Home",
        "tem_quantity_of_pass_fail" => "quantity of Pass | Fail",
        "tem_yield_of_shift" => "yield of shift",
        "tem_fail_record_details" => "Fail record details",
        "tem_query_400_records_of_cell" => "query 400 records of CELL",
        "tem_query_sn_history" => "query SN history (auto)",
        "tem_query_sn_history_all" => "query SN history (all )",
        "tem_key_name_of_bgibest" => "key name of Bgibest",
        "tem_port_config_of_terminal_server" => "port config of Router",
        "tem_line_0" => "line 0",
        "tem_line_1" => "line 1",
        "tem_line_2" => "line 2",
        "tem_line_3" => "line 3",
        "tem_submit" => "Submit",
        "tem_local_time" => "Local time",
        "tem_sort_able" => "you can click the table header to sort",
    };
    let c = hashmap! {
        "tem_language" => "zh-CN",
        "tem_day" => "白班",
        "tem_night" => "晚班",
        "tem_previous_day" => "前一天",
        "tem_previous_shift" => "前一班",
        "tem_today" => "今天",
        "tem_viewing_data" => "当前页面数据",
        "tem_colon" => "：",
        "tem_home" => "首页",
        "tem_quantity_of_pass_fail" => "每班测试 Pass | Fail 数量",
        "tem_yield_of_shift" => "每班良率",
        "tem_fail_record_details" => "每班 Fail 记录详细信息",
        "tem_query_400_records_of_cell" => "查询 CELL 最近400次记录",
        "tem_query_sn_history" => "查询 SN 的记录（仅自动化线）",
        "tem_query_sn_history_all" => "查询 SN 的记录（所有）",
        "tem_key_name_of_bgibest" => "Bgibest 各按键的名字",
        "tem_port_config_of_terminal_server" => "路由 Port 的使用情况",
        "tem_line_0" => "0线",
        "tem_line_1" => "1线",
        "tem_line_2" => "2线",
        "tem_line_3" => "3线",
        "tem_submit" => "查询",
        "tem_local_time" => "本地时间",
        "tem_sort_able" => "点击表头可以排序",
    };
    let v = hashmap! {
        "tem_language" => "vi-VN",
        "tem_day" => "DAY",
        "tem_night" => "NIGHT",
        "tem_previous_day" => "previous day",
        "tem_previous_shift" => "previous shift",
        "tem_today" => "today",
        "tem_viewing_data" => "Viewing data",
        "tem_colon" => ": ",
        "tem_home" => "Home",
        "tem_quantity_of_pass_fail" => "quantity of Pass | Fail",
        "tem_yield_of_shift" => "yield of shift",
        "tem_fail_record_details" => "Fail record details",
        "tem_query_400_records_of_cell" => "query 400 records of CELL",
        "tem_query_sn_history" => "query SN history (auto)",
        "tem_query_sn_history_all" => "query SN history (all )",
        "tem_key_name_of_bgibest" => "key name of Bgibest",
        "tem_port_config_of_terminal_server" => "port config of Router",
        "tem_line_0" => "line 0",
        "tem_line_1" => "line 1",
        "tem_line_2" => "line 2",
        "tem_line_3" => "line 3",
        "tem_submit" => "Submit",
        "tem_local_time" => "Local time",
        "tem_sort_able" => "you can click the table header to sort",
    };
    debug_assert_eq!(e.len(), c.len());
    debug_assert_eq!(e.len(), v.len());
    let mut map: HashMap<&'static str, HashMap<&'static str, &'static str>> = HashMap::new();
    map.insert("en-US", e);
    map.insert("zh-CN", c);
    map.insert("vi-VN", v);
    LANG_MAP.get_or_init(|| map);
}
