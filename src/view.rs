use std::string::ToString;
use crate::analysis::config::CONFIG;
use crate::analysis::{db, mytime};
use crate::analysis::lang_tran::LANG_MAP;
use crate::template;

use http::HeaderMap;
use axum::{extract, Json};
use axum::response::{IntoResponse, Redirect, Response};
use serde::{Deserialize, Serialize};
use regex::Regex;
use template::{CellBstTemplate, CellFstTemplate, StationYieldTemplate, HomepageTemplate, HtmlTemplate, KeynameTemplate,
               PortconfigTemplate, DetailBstTemplate, DetailFstTemplate, YiedlBstTemplate,
               YiedlFstTemplate, PfBstTemplate, PfFstTemplate, SnRecordTemplate};


const LANG_CODE: [&str; 3] = ["en-US", "zh-CN", "vi-VN"];
const LINES: [&str; 4] = ["bst1", "bst2", "fst1", "fst2"];
static DAY: &str = "DAY";
static NIGHT: &str = "NIGHT";

trait QueryParaValid {
    fn valid(&self) -> bool; //todo, use serde ?
}

pub async fn homepage() -> impl IntoResponse {
    let config = CONFIG.get().unwrap();
    let lang_map = LANG_MAP.get().unwrap();
    let lang_map = lang_map.get("en-US").unwrap();
    let bst1_db_path_exist = if config.bst1.bst_db.exists() { "" } else { " (not exist !)" };
    let bst2_db_path_exist = if config.bst2.bst_db.exists() { "" } else { " (not exist !)" };
    let fst1_lcd_db_path_exist = if config.fst1.lcd_db.exists() { "" } else { " (not exist !)" };
    let fst1_diag_db_path_exist = if config.fst1.diag_db.exists() { "" } else { " (not exist !)" };
    let fst1_key_db_path_exist = if config.fst1.key_db.exists() { "" } else { " (not exist !)" };
    let fst2_lcd_db_path_exist = if config.fst2.lcd_db.exists() { "" } else { " (not exist !)" };
    let fst2_diag_db_path_exist = if config.fst2.diag_db.exists() { "" } else { " (not exist !)" };
    let fst2_key_db_path_exist = if config.fst2.key_db.exists() { "" } else { " (not exist !)" };
    let tpl = HomepageTemplate {
        current_exe_path: config.current_exe_path.to_str().unwrap(),
        current_config_path: config.current_config_path.to_str().unwrap(),
        current_db_path: config.current_db_path.to_str().unwrap(),
        bst1_machine: &config.bst1.hostname,
        bst2_machine: &config.bst2.hostname,
        fst1_machine: &config.fst1.hostname,
        fst2_machine: &config.fst2.hostname,
        bst1_db_path: config.bst1.bst_db.to_str().unwrap(),
        bst2_db_path: config.bst2.bst_db.to_str().unwrap(),
        fst1_lcd_db_path: config.fst1.lcd_db.to_str().unwrap(),
        fst1_diag_db_path: config.fst1.diag_db.to_str().unwrap(),
        fst1_key_db_path: config.fst1.key_db.to_str().unwrap(),
        fst2_lcd_db_path: config.fst2.lcd_db.to_str().unwrap(),
        fst2_diag_db_path: config.fst2.diag_db.to_str().unwrap(),
        fst2_key_db_path: config.fst2.key_db.to_str().unwrap(),
        bst1_db_path_exist,
        bst2_db_path_exist,
        fst1_lcd_db_path_exist,
        fst1_diag_db_path_exist,
        fst1_key_db_path_exist,
        fst2_lcd_db_path_exist,
        fst2_diag_db_path_exist,
        fst2_key_db_path_exist,
        // base.html
        // all tem variant
        tem_language: lang_map.get("tem_language").unwrap(),
        tem_day: lang_map.get("tem_day").unwrap(),
        tem_night: lang_map.get("tem_night").unwrap(),
        tem_previous_day: lang_map.get("tem_previous_day").unwrap(),
        tem_previous_shift: lang_map.get("tem_previous_shift").unwrap(),
        tem_today: lang_map.get("tem_today").unwrap(),
        tem_viewing_data: lang_map.get("tem_viewing_data").unwrap(),
        tem_colon: lang_map.get("tem_colon").unwrap(),
        tem_home: lang_map.get("tem_home").unwrap(),
        tem_quantity_of_pass_fail: lang_map.get("tem_quantity_of_pass_fail").unwrap(),
        tem_yield_of_shift: lang_map.get("tem_yield_of_shift").unwrap(),
        tem_fail_record_details: lang_map.get("tem_fail_record_details").unwrap(),
        tem_query_400_records_of_cell: lang_map.get("tem_query_400_records_of_cell").unwrap(),
        tem_query_sn_history: lang_map.get("tem_query_sn_history").unwrap(),
        tem_query_sn_history_all: lang_map.get("tem_query_sn_history_all").unwrap(),
        tem_key_name_of_bgibest: lang_map.get("tem_key_name_of_bgibest").unwrap(),
        tem_port_config_of_terminal_server: lang_map
            .get("tem_port_config_of_terminal_server")
            .unwrap(),
        tem_line_0: lang_map.get("tem_line_0").unwrap(),
        tem_line_1: lang_map.get("tem_line_1").unwrap(),
        tem_line_2: lang_map.get("tem_line_2").unwrap(),
        tem_line_3: lang_map.get("tem_line_3").unwrap(),
        tem_submit: lang_map.get("tem_submit").unwrap(),
        tem_local_time: lang_map.get("tem_local_time").unwrap(),
        tem_sort_able: lang_map.get("tem_sort_able").unwrap(),
    };
    HtmlTemplate(tpl)
}

pub async fn line_page(extract::Path((lang, line)): extract::Path<(String, String)>) -> impl IntoResponse {
    let url = format!("/{lang}/{line}/pf_data");
    Redirect::to(&url).into_response()
}

pub async fn portconfig(extract::Path((lang, line)): extract::Path<(String, String)>) -> impl IntoResponse {
    assert!(LANG_CODE.contains(&lang.as_str()));
    let config = CONFIG.get().unwrap();
    let hostname = config.get_hostname(&line).unwrap();
    let hostname = hostname.to_owned();
    let lang_map = LANG_MAP.get().unwrap();
    let lang_map = lang_map.get(lang.as_str()).unwrap();
    let title = "Port Config".to_string();
    let tpl = PortconfigTemplate {
        title,
        line,
        hostname,
        update_time: mytime::now_vn(),
        tem_local_time: lang_map.get("tem_local_time").unwrap(),
        tem_language: lang_map.get("tem_language").unwrap(),
        tem_quantity_of_pass_fail: lang_map.get("tem_quantity_of_pass_fail").unwrap(),
        tem_yield_of_shift: lang_map.get("tem_yield_of_shift").unwrap(),
        tem_fail_record_details: lang_map.get("tem_fail_record_details").unwrap(),
        tem_query_400_records_of_cell: lang_map.get("tem_query_400_records_of_cell").unwrap(),
        tem_query_sn_history: lang_map.get("tem_query_sn_history").unwrap(),
        tem_query_sn_history_all: lang_map.get("tem_query_sn_history_all").unwrap(),
        tem_key_name_of_bgibest: lang_map.get("tem_key_name_of_bgibest").unwrap(),
        tem_port_config_of_terminal_server: lang_map
            .get("tem_port_config_of_terminal_server")
            .unwrap(),
        tem_line_1: lang_map.get("tem_line_1").unwrap(),
        tem_line_2: lang_map.get("tem_line_2").unwrap(),
        tem_line_3: lang_map.get("tem_line_3").unwrap(),
        tem_line_4: lang_map.get("tem_line_3").unwrap(),
    };
    HtmlTemplate(tpl)
}

pub async fn keyname(extract::Path((lang, line)): extract::Path<(String, String)>) -> impl IntoResponse {
    assert!(LANG_CODE.contains(&lang.as_str()));
    let config = CONFIG.get().unwrap();
    let hostname = config.get_hostname(&line).unwrap();
    let hostname = hostname.to_owned();
    let lang_map = LANG_MAP.get().unwrap();
    let lang_map = lang_map.get(lang.as_str()).unwrap();
    let title = "Key Name".to_string();
    let tpl = KeynameTemplate {
        title,
        line,
        hostname,
        update_time: mytime::now_vn(),
        tem_local_time: lang_map.get("tem_local_time").unwrap(),
        tem_language: lang_map.get("tem_language").unwrap(),
        tem_quantity_of_pass_fail: lang_map.get("tem_quantity_of_pass_fail").unwrap(),
        tem_yield_of_shift: lang_map.get("tem_yield_of_shift").unwrap(),
        tem_fail_record_details: lang_map.get("tem_fail_record_details").unwrap(),
        tem_query_400_records_of_cell: lang_map.get("tem_query_400_records_of_cell").unwrap(),
        tem_query_sn_history: lang_map.get("tem_query_sn_history").unwrap(),
        tem_query_sn_history_all: lang_map.get("tem_query_sn_history_all").unwrap(),
        tem_key_name_of_bgibest: lang_map.get("tem_key_name_of_bgibest").unwrap(),
        tem_port_config_of_terminal_server: lang_map
            .get("tem_port_config_of_terminal_server")
            .unwrap(),
        tem_line_1: lang_map.get("tem_line_1").unwrap(),
        tem_line_2: lang_map.get("tem_line_2").unwrap(),
        tem_line_3: lang_map.get("tem_line_3").unwrap(),
    };
    HtmlTemplate(tpl)
}

#[derive(Debug, Deserialize)]
pub struct QueryCellParams {
    cell: Option<String>,
}

impl QueryParaValid for QueryCellParams {
    fn valid(&self) -> bool {
        if let Some(cell) = &self.cell {
            db::cell2station(cell).is_some()
        } else {
            false
        }
    }
}

pub async fn query_cell(
    extract::Path((lang, line)): extract::Path<(String, String)>,
    extract::Query(query_params): extract::Query<QueryCellParams>,
) -> Response {
    assert!(LANG_CODE.contains(&lang.as_str()));
    assert!(LINES.contains(&line.as_str()));
    let query_count = 400;
    let config = CONFIG.get().unwrap();
    let hostname = config.get_hostname(&line).unwrap();
    let hostname = hostname.to_owned();
    let update_time = mytime::now_vn();
    let lang_map = LANG_MAP.get().unwrap();
    let lang_map = lang_map.get(lang.as_str()).unwrap();
    let title = "Query Cell".to_string();
    if query_params.valid() {
        let cell = query_params.cell.unwrap();
        let (res_yield, fail_data) = db::query_cell(&line, &cell, query_count).unwrap();
        let resp = if line.contains("bst") {
            let tpl =
                CellBstTemplate {
                    query_count,
                    cell: &cell,
                    res_yield,
                    record: fail_data,
                    // base.html
                    title,
                    line,
                    hostname,
                    update_time,
                    // all tem variant
                    tem_language: lang_map.get("tem_language").unwrap(),
                    tem_day: lang_map.get("tem_day").unwrap(),
                    tem_night: lang_map.get("tem_night").unwrap(),
                    tem_previous_day: lang_map.get("tem_previous_day").unwrap(),
                    tem_previous_shift: lang_map.get("tem_previous_shift").unwrap(),
                    tem_today: lang_map.get("tem_today").unwrap(),
                    tem_viewing_data: lang_map.get("tem_viewing_data").unwrap(),
                    tem_colon: lang_map.get("tem_colon").unwrap(),
                    tem_home: lang_map.get("tem_home").unwrap(),
                    tem_quantity_of_pass_fail: lang_map.get("tem_quantity_of_pass_fail").unwrap(),
                    tem_yield_of_shift: lang_map.get("tem_yield_of_shift").unwrap(),
                    tem_fail_record_details: lang_map.get("tem_fail_record_details").unwrap(),
                    tem_query_400_records_of_cell: lang_map.get("tem_query_400_records_of_cell").unwrap(),
                    tem_query_sn_history: lang_map.get("tem_query_sn_history").unwrap(),
                    tem_query_sn_history_all: lang_map.get("tem_query_sn_history_all").unwrap(),
                    tem_key_name_of_bgibest: lang_map.get("tem_key_name_of_bgibest").unwrap(),
                    tem_port_config_of_terminal_server: lang_map
                        .get("tem_port_config_of_terminal_server")
                        .unwrap(),
                    tem_line_0: lang_map.get("tem_line_0").unwrap(),
                    tem_line_1: lang_map.get("tem_line_1").unwrap(),
                    tem_line_2: lang_map.get("tem_line_2").unwrap(),
                    tem_line_3: lang_map.get("tem_line_3").unwrap(),
                    tem_submit: lang_map.get("tem_submit").unwrap(),
                    tem_local_time: lang_map.get("tem_local_time").unwrap(),
                    tem_sort_able: lang_map.get("tem_sort_able").unwrap(),
                };
            HtmlTemplate(tpl).into_response()
        } else {
            let tpl =
                CellFstTemplate {
                    query_count,
                    cell: &cell,
                    res_yield,
                    record: fail_data,
                    // base.html
                    title,
                    line,
                    hostname,
                    update_time,
                    // all tem variant
                    tem_language: lang_map.get("tem_language").unwrap(),
                    tem_day: lang_map.get("tem_day").unwrap(),
                    tem_night: lang_map.get("tem_night").unwrap(),
                    tem_previous_day: lang_map.get("tem_previous_day").unwrap(),
                    tem_previous_shift: lang_map.get("tem_previous_shift").unwrap(),
                    tem_today: lang_map.get("tem_today").unwrap(),
                    tem_viewing_data: lang_map.get("tem_viewing_data").unwrap(),
                    tem_colon: lang_map.get("tem_colon").unwrap(),
                    tem_home: lang_map.get("tem_home").unwrap(),
                    tem_quantity_of_pass_fail: lang_map.get("tem_quantity_of_pass_fail").unwrap(),
                    tem_yield_of_shift: lang_map.get("tem_yield_of_shift").unwrap(),
                    tem_fail_record_details: lang_map.get("tem_fail_record_details").unwrap(),
                    tem_query_400_records_of_cell: lang_map.get("tem_query_400_records_of_cell").unwrap(),
                    tem_query_sn_history: lang_map.get("tem_query_sn_history").unwrap(),
                    tem_query_sn_history_all: lang_map.get("tem_query_sn_history_all").unwrap(),
                    tem_key_name_of_bgibest: lang_map.get("tem_key_name_of_bgibest").unwrap(),
                    tem_port_config_of_terminal_server: lang_map
                        .get("tem_port_config_of_terminal_server")
                        .unwrap(),
                    tem_line_0: lang_map.get("tem_line_0").unwrap(),
                    tem_line_1: lang_map.get("tem_line_1").unwrap(),
                    tem_line_2: lang_map.get("tem_line_2").unwrap(),
                    tem_line_3: lang_map.get("tem_line_3").unwrap(),
                    tem_submit: lang_map.get("tem_submit").unwrap(),
                    tem_local_time: lang_map.get("tem_local_time").unwrap(),
                    tem_sort_able: lang_map.get("tem_sort_able").unwrap(),
                };
            HtmlTemplate(tpl).into_response()
        };
        return resp;
    }
    let stations = if line.contains("bst") { vec!["BST"] } else { vec!["LCDLED", "DIAG", "KEYPAD"] };
    let mut station_yield = Vec::new();
    for s in stations {
        let mut a = db::query_station_yield(&line, s, query_count).unwrap();
        station_yield.append(&mut a);
    }
    let tpl = StationYieldTemplate {
        station_yield,
        // base.html
        title,
        line,
        hostname,
        update_time,
        // all tem variant
        tem_language: lang_map.get("tem_language").unwrap(),
        tem_day: lang_map.get("tem_day").unwrap(),
        tem_night: lang_map.get("tem_night").unwrap(),
        tem_previous_day: lang_map.get("tem_previous_day").unwrap(),
        tem_previous_shift: lang_map.get("tem_previous_shift").unwrap(),
        tem_today: lang_map.get("tem_today").unwrap(),
        tem_viewing_data: lang_map.get("tem_viewing_data").unwrap(),
        tem_colon: lang_map.get("tem_colon").unwrap(),
        tem_home: lang_map.get("tem_home").unwrap(),
        tem_quantity_of_pass_fail: lang_map.get("tem_quantity_of_pass_fail").unwrap(),
        tem_yield_of_shift: lang_map.get("tem_yield_of_shift").unwrap(),
        tem_fail_record_details: lang_map.get("tem_fail_record_details").unwrap(),
        tem_query_400_records_of_cell: lang_map.get("tem_query_400_records_of_cell").unwrap(),
        tem_query_sn_history: lang_map.get("tem_query_sn_history").unwrap(),
        tem_query_sn_history_all: lang_map.get("tem_query_sn_history_all").unwrap(),
        tem_key_name_of_bgibest: lang_map.get("tem_key_name_of_bgibest").unwrap(),
        tem_port_config_of_terminal_server: lang_map
            .get("tem_port_config_of_terminal_server")
            .unwrap(),
        tem_line_0: lang_map.get("tem_line_0").unwrap(),
        tem_line_1: lang_map.get("tem_line_1").unwrap(),
        tem_line_2: lang_map.get("tem_line_2").unwrap(),
        tem_line_3: lang_map.get("tem_line_3").unwrap(),
        tem_submit: lang_map.get("tem_submit").unwrap(),
        tem_local_time: lang_map.get("tem_local_time").unwrap(),
        tem_sort_able: lang_map.get("tem_sort_able").unwrap(),
    };
    HtmlTemplate(tpl).into_response()
}

#[derive(Debug, Deserialize)]
pub struct QueryDateParams {
    querydate: Option<String>,
    shift: Option<String>,
}

impl QueryDateParams {
    pub fn des(&self) -> Option<mytime::DateShift> {
        let re_date: Regex = Regex::new(r"^(202\d)-(0[1-9]|1[012])-(0[1-9]|[12]\d|3[01])$").unwrap(); // todo, move to global var
        let shift = match &self.shift {
            Some(s) => {
                if s == DAY {
                    mytime::Shift::Day
                } else if s == NIGHT {
                    mytime::Shift::Night
                } else {
                    return None;
                }
            }
            None => { return None; }
        };
        if let Some(date) = &self.querydate {
            if let Some(cap) = re_date.captures(date) {
                let (year, month, day) = (&cap[1], &cap[2], &cap[3]);
                let (year, month, day) = (year.parse().unwrap(), month.parse().unwrap(), day.parse().unwrap());
                return Some(mytime::DateShift(year, month, day, shift));
            }
        };
        None
    }
}

impl QueryParaValid for QueryDateParams {
    fn valid(&self) -> bool {
        let (mut flag1, mut flag2) = (false, false);
        let re_date: Regex = Regex::new(r"^(202\d)-(0[1-9]|1[012])-(0[1-9]|[12]\d|3[01])$").unwrap(); // todo, move to global var
        if let Some(querydate) = &self.querydate {
            if !re_date.is_match(querydate) {
                flag1 = true;
            }
        }
        if let Some(shift) = &self.shift {
            if shift == DAY || shift == NIGHT {
                flag2 = true
            }
        }
        flag1 && flag2
    }
}

pub async fn fail_detail(
    extract::Path((lang, line)): extract::Path<(String, String)>,
    extract::Query(query_params): extract::Query<QueryDateParams>,
) -> Response {
    assert!(LANG_CODE.contains(&lang.as_str()));
    assert!(LINES.contains(&line.as_str()));
    let config = CONFIG.get().unwrap();
    let hostname = config.get_hostname(&line).unwrap();
    let hostname = hostname.to_owned();
    let update_time = mytime::now_vn();
    let lang_map = LANG_MAP.get().unwrap();
    let lang_map = lang_map.get(lang.as_str()).unwrap();
    let title = "Fail Detail".to_string();
    let dateshift = query_params.des();
    if dateshift.is_none() {
        let current_shift = mytime::current_shift();
        let (date, shift) = (current_shift.date(), current_shift.shift());
        let url = format!("/{lang}/{line}/fail_detail/?querydate={date}&shift={shift}");
        return Redirect::to(&url).into_response();
    };
    let dateshift = dateshift.unwrap();
    let date = dateshift.date();
    let shift = dateshift.shift().to_string();
    if line.contains("bst") {
        let bst = db::fail_detail(&line, "BST", &dateshift).unwrap();
        let tpl = DetailBstTemplate {
            data_day: &date,
            data_shift: &shift,
            bst,
            // base.html
            title,
            line,
            hostname,
            update_time,
            // all tem variant
            tem_language: lang_map.get("tem_language").unwrap(),
            tem_day: lang_map.get("tem_day").unwrap(),
            tem_night: lang_map.get("tem_night").unwrap(),
            tem_previous_day: lang_map.get("tem_previous_day").unwrap(),
            tem_previous_shift: lang_map.get("tem_previous_shift").unwrap(),
            tem_today: lang_map.get("tem_today").unwrap(),
            tem_viewing_data: lang_map.get("tem_viewing_data").unwrap(),
            tem_colon: lang_map.get("tem_colon").unwrap(),
            tem_home: lang_map.get("tem_home").unwrap(),
            tem_quantity_of_pass_fail: lang_map.get("tem_quantity_of_pass_fail").unwrap(),
            tem_yield_of_shift: lang_map.get("tem_yield_of_shift").unwrap(),
            tem_fail_record_details: lang_map.get("tem_fail_record_details").unwrap(),
            tem_query_400_records_of_cell: lang_map.get("tem_query_400_records_of_cell").unwrap(),
            tem_query_sn_history: lang_map.get("tem_query_sn_history").unwrap(),
            tem_query_sn_history_all: lang_map.get("tem_query_sn_history_all").unwrap(),
            tem_key_name_of_bgibest: lang_map.get("tem_key_name_of_bgibest").unwrap(),
            tem_port_config_of_terminal_server: lang_map
                .get("tem_port_config_of_terminal_server")
                .unwrap(),
            tem_line_0: lang_map.get("tem_line_0").unwrap(),
            tem_line_1: lang_map.get("tem_line_1").unwrap(),
            tem_line_2: lang_map.get("tem_line_2").unwrap(),
            tem_line_3: lang_map.get("tem_line_3").unwrap(),
            tem_submit: lang_map.get("tem_submit").unwrap(),
            tem_local_time: lang_map.get("tem_local_time").unwrap(),
            tem_sort_able: lang_map.get("tem_sort_able").unwrap(),
        };
        HtmlTemplate(tpl).into_response()
    } else {
        let lcdled = db::fail_detail(&line, "LCDLED", &dateshift).unwrap();
        let diag = db::fail_detail(&line, "DIAG", &dateshift).unwrap();
        let keypad = db::fail_detail(&line, "KEYPAD", &dateshift).unwrap();
        let tpl = DetailFstTemplate {
            data_day: &date,
            data_shift: &shift,
            lcdled,
            diag,
            keypad,
            // base.html
            title,
            line,
            hostname,
            update_time,
            // all tem variant
            tem_language: lang_map.get("tem_language").unwrap(),
            tem_day: lang_map.get("tem_day").unwrap(),
            tem_night: lang_map.get("tem_night").unwrap(),
            tem_previous_day: lang_map.get("tem_previous_day").unwrap(),
            tem_previous_shift: lang_map.get("tem_previous_shift").unwrap(),
            tem_today: lang_map.get("tem_today").unwrap(),
            tem_viewing_data: lang_map.get("tem_viewing_data").unwrap(),
            tem_colon: lang_map.get("tem_colon").unwrap(),
            tem_home: lang_map.get("tem_home").unwrap(),
            tem_quantity_of_pass_fail: lang_map.get("tem_quantity_of_pass_fail").unwrap(),
            tem_yield_of_shift: lang_map.get("tem_yield_of_shift").unwrap(),
            tem_fail_record_details: lang_map.get("tem_fail_record_details").unwrap(),
            tem_query_400_records_of_cell: lang_map.get("tem_query_400_records_of_cell").unwrap(),
            tem_query_sn_history: lang_map.get("tem_query_sn_history").unwrap(),
            tem_query_sn_history_all: lang_map.get("tem_query_sn_history_all").unwrap(),
            tem_key_name_of_bgibest: lang_map.get("tem_key_name_of_bgibest").unwrap(),
            tem_port_config_of_terminal_server: lang_map
                .get("tem_port_config_of_terminal_server")
                .unwrap(),
            tem_line_0: lang_map.get("tem_line_0").unwrap(),
            tem_line_1: lang_map.get("tem_line_1").unwrap(),
            tem_line_2: lang_map.get("tem_line_2").unwrap(),
            tem_line_3: lang_map.get("tem_line_3").unwrap(),
            tem_submit: lang_map.get("tem_submit").unwrap(),
            tem_local_time: lang_map.get("tem_local_time").unwrap(),
            tem_sort_able: lang_map.get("tem_sort_able").unwrap(),
        };
        HtmlTemplate(tpl).into_response()
    }
}

pub async fn pf_data(extract::Path((lang, line)): extract::Path<(String, String)>,
                     extract::Query(query_params): extract::Query<QueryDateParams>,
) -> Response {
    assert!(LANG_CODE.contains(&lang.as_str()));
    assert!(LINES.contains(&line.as_str()));
    let config = CONFIG.get().unwrap();
    let hostname = config.get_hostname(&line).unwrap();
    let hostname = hostname.to_owned();
    let update_time = mytime::now_vn();
    let lang_map = LANG_MAP.get().unwrap();
    let lang_map = lang_map.get(lang.as_str()).unwrap();
    let title = "Pass | Fail data".to_string();
    let dateshift = query_params.des();
    if dateshift.is_none() {
        let current_shift = mytime::current_shift();
        let (date, shift) = (current_shift.date(), current_shift.shift());
        let url = format!("/{lang}/{line}/pf_data/?querydate={date}&shift={shift}");
        println!("pf_data, redirect to: {url}");
        return Redirect::to(&url).into_response();
    };
    let dateshift = dateshift.unwrap();
    let date = dateshift.date();
    let shift = dateshift.shift().to_string();
    let hours_str = mytime::hours_str(dateshift.shift(), true);
    let ts_per_hour = mytime::ts_per_hour_shift(&dateshift);
    if line.contains("bst") {
        let bst = db::pf_data(&line, "BST", ts_per_hour, &hours_str).unwrap();
        let tpl = PfBstTemplate {
            data_day: &date,
            data_shift: &shift,
            bst,
            // base.html
            title,
            line,
            hostname,
            update_time,
            // all tem variant
            tem_language: lang_map.get("tem_language").unwrap(),
            tem_day: lang_map.get("tem_day").unwrap(),
            tem_night: lang_map.get("tem_night").unwrap(),
            tem_previous_day: lang_map.get("tem_previous_day").unwrap(),
            tem_previous_shift: lang_map.get("tem_previous_shift").unwrap(),
            tem_today: lang_map.get("tem_today").unwrap(),
            tem_viewing_data: lang_map.get("tem_viewing_data").unwrap(),
            tem_colon: lang_map.get("tem_colon").unwrap(),
            tem_home: lang_map.get("tem_home").unwrap(),
            tem_quantity_of_pass_fail: lang_map.get("tem_quantity_of_pass_fail").unwrap(),
            tem_yield_of_shift: lang_map.get("tem_yield_of_shift").unwrap(),
            tem_fail_record_details: lang_map.get("tem_fail_record_details").unwrap(),
            tem_query_400_records_of_cell: lang_map.get("tem_query_400_records_of_cell").unwrap(),
            tem_query_sn_history: lang_map.get("tem_query_sn_history").unwrap(),
            tem_query_sn_history_all: lang_map.get("tem_query_sn_history_all").unwrap(),
            tem_key_name_of_bgibest: lang_map.get("tem_key_name_of_bgibest").unwrap(),
            tem_port_config_of_terminal_server: lang_map
                .get("tem_port_config_of_terminal_server")
                .unwrap(),
            tem_line_0: lang_map.get("tem_line_0").unwrap(),
            tem_line_1: lang_map.get("tem_line_1").unwrap(),
            tem_line_2: lang_map.get("tem_line_2").unwrap(),
            tem_line_3: lang_map.get("tem_line_3").unwrap(),
            tem_submit: lang_map.get("tem_submit").unwrap(),
            tem_local_time: lang_map.get("tem_local_time").unwrap(),
            tem_sort_able: lang_map.get("tem_sort_able").unwrap(),
        };
        HtmlTemplate(tpl).into_response()
    } else {
        let lcdled = db::pf_data(&line, "LCDLED", ts_per_hour, &hours_str).unwrap();
        let diag = db::pf_data(&line, "DIAG", ts_per_hour, &hours_str).unwrap();
        let keypad = db::pf_data(&line, "KEYPAD", ts_per_hour, &hours_str).unwrap();
        let tpl = PfFstTemplate {
            data_day: &date,
            data_shift: &shift,
            lcdled,
            diag,
            keypad,
            // base.html
            title,
            line,
            hostname,
            update_time,
            // all tem variant
            tem_language: lang_map.get("tem_language").unwrap(),
            tem_day: lang_map.get("tem_day").unwrap(),
            tem_night: lang_map.get("tem_night").unwrap(),
            tem_previous_day: lang_map.get("tem_previous_day").unwrap(),
            tem_previous_shift: lang_map.get("tem_previous_shift").unwrap(),
            tem_today: lang_map.get("tem_today").unwrap(),
            tem_viewing_data: lang_map.get("tem_viewing_data").unwrap(),
            tem_colon: lang_map.get("tem_colon").unwrap(),
            tem_home: lang_map.get("tem_home").unwrap(),
            tem_quantity_of_pass_fail: lang_map.get("tem_quantity_of_pass_fail").unwrap(),
            tem_yield_of_shift: lang_map.get("tem_yield_of_shift").unwrap(),
            tem_fail_record_details: lang_map.get("tem_fail_record_details").unwrap(),
            tem_query_400_records_of_cell: lang_map.get("tem_query_400_records_of_cell").unwrap(),
            tem_query_sn_history: lang_map.get("tem_query_sn_history").unwrap(),
            tem_query_sn_history_all: lang_map.get("tem_query_sn_history_all").unwrap(),
            tem_key_name_of_bgibest: lang_map.get("tem_key_name_of_bgibest").unwrap(),
            tem_port_config_of_terminal_server: lang_map
                .get("tem_port_config_of_terminal_server")
                .unwrap(),
            tem_line_0: lang_map.get("tem_line_0").unwrap(),
            tem_line_1: lang_map.get("tem_line_1").unwrap(),
            tem_line_2: lang_map.get("tem_line_2").unwrap(),
            tem_line_3: lang_map.get("tem_line_3").unwrap(),
            tem_submit: lang_map.get("tem_submit").unwrap(),
            tem_local_time: lang_map.get("tem_local_time").unwrap(),
            tem_sort_able: lang_map.get("tem_sort_able").unwrap(),
        };
        HtmlTemplate(tpl).into_response()
    }
}

pub async fn day_yield(extract::Path((lang, line)): extract::Path<(String, String)>,
                       extract::Query(query_params): extract::Query<QueryDateParams>) -> Response {
    assert!(LANG_CODE.contains(&lang.as_str()));
    assert!(LINES.contains(&line.as_str()));
    let config = CONFIG.get().unwrap();
    let hostname = config.get_hostname(&line).unwrap();
    let hostname = hostname.to_owned();
    let update_time = mytime::now_vn();
    let lang_map = LANG_MAP.get().unwrap();
    let lang_map = lang_map.get(lang.as_str()).unwrap();
    let title = "Day Yield".to_string();
    let dateshift = query_params.des();
    if dateshift.is_none() {
        let current_shift = mytime::current_shift();
        let (date, shift) = (current_shift.date(), current_shift.shift());
        let url = format!("/{lang}/{line}/day_yield/?querydate={date}&shift={shift}");
        return Redirect::to(&url).into_response();
    };
    let dateshift = dateshift.unwrap();
    let date = dateshift.date();
    let shift = dateshift.shift().to_string();
    let hours_str = mytime::hours_str(dateshift.shift(), true);
    let ts_per_hour = mytime::ts_per_hour_shift(&dateshift);
    if line.contains("bst") {
        let bst = db::day_yield(&line, "BST", ts_per_hour, &hours_str).unwrap();
        let tpl = YiedlBstTemplate {
            data_day: &date,
            data_shift: &shift,
            bst,
            // base.html
            title,
            line,
            hostname,
            update_time,
            // all tem variant
            tem_language: lang_map.get("tem_language").unwrap(),
            tem_day: lang_map.get("tem_day").unwrap(),
            tem_night: lang_map.get("tem_night").unwrap(),
            tem_previous_day: lang_map.get("tem_previous_day").unwrap(),
            tem_previous_shift: lang_map.get("tem_previous_shift").unwrap(),
            tem_today: lang_map.get("tem_today").unwrap(),
            tem_viewing_data: lang_map.get("tem_viewing_data").unwrap(),
            tem_colon: lang_map.get("tem_colon").unwrap(),
            tem_home: lang_map.get("tem_home").unwrap(),
            tem_quantity_of_pass_fail: lang_map.get("tem_quantity_of_pass_fail").unwrap(),
            tem_yield_of_shift: lang_map.get("tem_yield_of_shift").unwrap(),
            tem_fail_record_details: lang_map.get("tem_fail_record_details").unwrap(),
            tem_query_400_records_of_cell: lang_map.get("tem_query_400_records_of_cell").unwrap(),
            tem_query_sn_history: lang_map.get("tem_query_sn_history").unwrap(),
            tem_query_sn_history_all: lang_map.get("tem_query_sn_history_all").unwrap(),
            tem_key_name_of_bgibest: lang_map.get("tem_key_name_of_bgibest").unwrap(),
            tem_port_config_of_terminal_server: lang_map
                .get("tem_port_config_of_terminal_server")
                .unwrap(),
            tem_line_0: lang_map.get("tem_line_0").unwrap(),
            tem_line_1: lang_map.get("tem_line_1").unwrap(),
            tem_line_2: lang_map.get("tem_line_2").unwrap(),
            tem_line_3: lang_map.get("tem_line_3").unwrap(),
            tem_submit: lang_map.get("tem_submit").unwrap(),
            tem_local_time: lang_map.get("tem_local_time").unwrap(),
            tem_sort_able: lang_map.get("tem_sort_able").unwrap(),
        };
        HtmlTemplate(tpl).into_response()
    } else {
        let lcdled = db::day_yield(&line, "LCDLED", ts_per_hour, &hours_str).unwrap();
        let diag = db::day_yield(&line, "DIAG", ts_per_hour, &hours_str).unwrap();
        let keypad = db::day_yield(&line, "KEYPAD", ts_per_hour, &hours_str).unwrap();
        let tpl = YiedlFstTemplate {
            data_day: &date,
            data_shift: &shift,
            lcdled,
            diag,
            keypad,
            // base.html
            title,
            line,
            hostname,
            update_time,
            // all tem variant
            tem_language: lang_map.get("tem_language").unwrap(),
            tem_day: lang_map.get("tem_day").unwrap(),
            tem_night: lang_map.get("tem_night").unwrap(),
            tem_previous_day: lang_map.get("tem_previous_day").unwrap(),
            tem_previous_shift: lang_map.get("tem_previous_shift").unwrap(),
            tem_today: lang_map.get("tem_today").unwrap(),
            tem_viewing_data: lang_map.get("tem_viewing_data").unwrap(),
            tem_colon: lang_map.get("tem_colon").unwrap(),
            tem_home: lang_map.get("tem_home").unwrap(),
            tem_quantity_of_pass_fail: lang_map.get("tem_quantity_of_pass_fail").unwrap(),
            tem_yield_of_shift: lang_map.get("tem_yield_of_shift").unwrap(),
            tem_fail_record_details: lang_map.get("tem_fail_record_details").unwrap(),
            tem_query_400_records_of_cell: lang_map.get("tem_query_400_records_of_cell").unwrap(),
            tem_query_sn_history: lang_map.get("tem_query_sn_history").unwrap(),
            tem_query_sn_history_all: lang_map.get("tem_query_sn_history_all").unwrap(),
            tem_key_name_of_bgibest: lang_map.get("tem_key_name_of_bgibest").unwrap(),
            tem_port_config_of_terminal_server: lang_map
                .get("tem_port_config_of_terminal_server")
                .unwrap(),
            tem_line_0: lang_map.get("tem_line_0").unwrap(),
            tem_line_1: lang_map.get("tem_line_1").unwrap(),
            tem_line_2: lang_map.get("tem_line_2").unwrap(),
            tem_line_3: lang_map.get("tem_line_3").unwrap(),
            tem_submit: lang_map.get("tem_submit").unwrap(),
            tem_local_time: lang_map.get("tem_local_time").unwrap(),
            tem_sort_able: lang_map.get("tem_sort_able").unwrap(),
        };
        HtmlTemplate(tpl).into_response()
    }
}

pub async fn pre_day(
    extract::Path((lang, line, item)): extract::Path<(String, String, String)>,
    header_map: HeaderMap) -> Response {
    assert!(LANG_CODE.contains(&lang.as_str()));
    assert!(LINES.contains(&line.as_str()));
    let referer = header_map.get("referer");
    match referer {
        None => {
            let cur = mytime::current_shift();
            let pre = cur.pre_day();
            let (date, shift) = (pre.date(), pre.shift());
            let url = format!("/{lang}/{line}/{item}/?querydate={date}&shift={shift}");
            println!("cap4 non redirect to: {url}");
            Redirect::to(&url).into_response()
        }
        Some(refer) => {
            // http://localhost:7890/en-US/fst2/fail_detail
            // http://localhost:7890/en-US/fst2/fail_detail/?querydate=2024-01-04&shift=DAY
            let re_url = Regex::new(r"http://[\w.]+:\d+?/(\w\w-\w\w)/(\w+)/(\w+)(/\?querydate=((20\d{2})-(0\d|1[12])-([012]\d|3[01]))&shift=(\w+))?$").unwrap();
            if let Some(cap) = re_url.captures(refer.to_str().unwrap()) {
                let (lang, line, item) = (&cap[1], &cap[2], &cap[3]);
                match cap.get(4) {
                    None => {
                        let cur = mytime::current_shift();
                        let pre = cur.pre_day();
                        let (date, shift) = (pre.date(), pre.shift());
                        let url = format!("/{lang}/{line}/{item}/?querydate={date}&shift={shift}");
                        println!("cap4 non redirect to: {url}");
                        return Redirect::to(&url).into_response();
                    }
                    Some(_) => {
                        let (year, month, day, shift) = (&cap[6], &cap[7], &cap[8], &cap[9]);
                        let date = mytime::pre_day_str2date(year, month, day);
                        let url = format!("/{lang}/{line}/{item}/?querydate={date}&shift={shift}");
                        println!("cap4 some redirect to: {url}");
                        return Redirect::to(&url).into_response();
                    }
                }
            };
            panic!("refer regex verify fail!");
        }
    }
}

pub async fn pre_shift(
    extract::Path((lang, line, item)): extract::Path<(String, String, String)>,
    header_map: HeaderMap) -> Response {
    assert!(LANG_CODE.contains(&lang.as_str()));
    assert!(LINES.contains(&line.as_str()));
    let referer = header_map.get("referer");
    match referer {
        None => {
            let cur = mytime::current_shift();
            let pre = cur.pre_shift();
            let (date, shift) = (pre.date(), pre.shift());
            let url = format!("/{lang}/{line}/{item}/?querydate={date}&shift={shift}");
            println!("cap4 non redirect to: {url}");
            Redirect::to(&url).into_response()
        }
        Some(refer) => {
            // http://localhost:7890/en-US/fst2/fail_detail
            // http://localhost:7890/en-US/fst2/fail_detail/?querydate=2024-01-04&shift=DAY
            let re_url = Regex::new(r"http://[\w.]+:\d+?/(\w\w-\w\w)/(\w+)/(\w+)(/\?querydate=((20\d{2})-(0\d|1[12])-([012]\d|3[01]))&shift=(\w+))?$").unwrap();
            if let Some(cap) = re_url.captures(refer.to_str().unwrap()) {
                let (lang, line, item) = (&cap[1], &cap[2], &cap[3]);
                match cap.get(4) {
                    None => {
                        let cur = mytime::current_shift();
                        let pre = cur.pre_shift();
                        let (date, shift) = (pre.date(), pre.shift());
                        let url = format!("/{lang}/{line}/{item}/?querydate={date}&shift={shift}");
                        println!("cap4 non redirect to: {url}");
                        return Redirect::to(&url).into_response();
                    }
                    Some(_) => {
                        let (year, month, day, shift) = (&cap[6], &cap[7], &cap[8], &cap[9]);
                        let (date, shift) = mytime::pre_shift_str2date(year, month, day, shift);
                        let url = format!("/{lang}/{line}/{item}/?querydate={date}&shift={shift}");
                        println!("cap4 some redirect to: {url}");
                        return Redirect::to(&url).into_response();
                    }
                }
            };
            panic!("refer regex verify fail!");
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QuerySnParams {
    sn: Option<String>,
}

pub async fn query_sn(extract::Path((lang, line)): extract::Path<(String, String)>,
                      extract::Query(query_params): extract::Query<QuerySnParams>, ) -> impl IntoResponse {
    assert!(LANG_CODE.contains(&lang.as_str()));
    assert!(LINES.contains(&line.as_str()));
    let config = CONFIG.get().unwrap();
    let hostname = config.get_hostname(&line).unwrap();
    let hostname = hostname.to_owned();
    let update_time = mytime::now_vn();
    let lang_map = LANG_MAP.get().unwrap();
    let lang_map = lang_map.get(lang.as_str()).unwrap();
    let title = "Query Sn".to_string();
    let sn = match query_params.sn {
        Some(_sn) => {
            let sn = _sn.trim().to_uppercase();
            let re_sn = Regex::new(r"^[A-Z0-9]{11}$").unwrap();
            if re_sn.is_match(&sn) && sn != "FCH11111111" || sn != "FCH12345678" {
                Some(sn)
            } else {
                None
            }
        }
        _ => None
    };
    let record = match sn {
        Some(sn) => {
            let r = db::sn_record(sn).unwrap();
            if !r.is_empty() {
                r
            } else {
                vec![("".to_string(), "".to_string(), "".to_string(), "".to_string(),
                      "", "".to_string(), "", "".to_string(), "".to_string(), "".to_string())]
            }
        }
        None => vec![("".to_string(), "".to_string(), "".to_string(), "".to_string(),
                      "", "".to_string(), "", "".to_string(), "".to_string(), "".to_string())]
    };
    let tpl = SnRecordTemplate {
        record,
        // base.html
        title,
        line,
        hostname,
        update_time,
        // all tem variant
        tem_language: lang_map.get("tem_language").unwrap(),
        tem_day: lang_map.get("tem_day").unwrap(),
        tem_night: lang_map.get("tem_night").unwrap(),
        tem_previous_day: lang_map.get("tem_previous_day").unwrap(),
        tem_previous_shift: lang_map.get("tem_previous_shift").unwrap(),
        tem_today: lang_map.get("tem_today").unwrap(),
        tem_viewing_data: lang_map.get("tem_viewing_data").unwrap(),
        tem_colon: lang_map.get("tem_colon").unwrap(),
        tem_home: lang_map.get("tem_home").unwrap(),
        tem_quantity_of_pass_fail: lang_map.get("tem_quantity_of_pass_fail").unwrap(),
        tem_yield_of_shift: lang_map.get("tem_yield_of_shift").unwrap(),
        tem_fail_record_details: lang_map.get("tem_fail_record_details").unwrap(),
        tem_query_400_records_of_cell: lang_map.get("tem_query_400_records_of_cell").unwrap(),
        tem_query_sn_history: lang_map.get("tem_query_sn_history").unwrap(),
        tem_query_sn_history_all: lang_map.get("tem_query_sn_history_all").unwrap(),
        tem_key_name_of_bgibest: lang_map.get("tem_key_name_of_bgibest").unwrap(),
        tem_port_config_of_terminal_server: lang_map
            .get("tem_port_config_of_terminal_server")
            .unwrap(),
        tem_line_0: lang_map.get("tem_line_0").unwrap(),
        tem_line_1: lang_map.get("tem_line_1").unwrap(),
        tem_line_2: lang_map.get("tem_line_2").unwrap(),
        tem_line_3: lang_map.get("tem_line_3").unwrap(),
        tem_submit: lang_map.get("tem_submit").unwrap(),
        tem_local_time: lang_map.get("tem_local_time").unwrap(),
        tem_sort_able: lang_map.get("tem_sort_able").unwrap(),
    };
    HtmlTemplate(tpl)
}

#[derive(Serialize)]
pub struct CurShift {
    date: String,
    shift: String,
}

pub async fn json_today() -> Json<CurShift> {
    let cur = mytime::current_shift();
    let cs = CurShift { date: cur.date(), shift: cur.shift().to_string() };
    Json(cs)
}