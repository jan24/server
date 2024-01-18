use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use crate::analysis::db::{Rcell, Rdetail, Rpf, Rrecord};

mod filters {
    pub fn zero2space<T: std::fmt::Display>(s: T) -> askama::Result<String> {
        let s = s.to_string();
        if s == "0" {
            Ok("".to_string())
        } else {
            Ok(s)
        }
    }
}

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
    where
        T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "homepage.html")]
pub struct HomepageTemplate<'a> {
    pub current_exe_path: &'a str,
    pub current_config_path: &'a str,
    pub current_db_path: &'a str,
    pub bst1_machine: &'a str,
    pub bst2_machine: &'a str,
    pub fst1_machine: &'a str,
    pub fst2_machine: &'a str,
    pub bst1_db_path: &'a str,
    pub bst2_db_path: &'a str,
    pub fst1_lcd_db_path: &'a str,
    pub fst1_diag_db_path: &'a str,
    pub fst1_key_db_path: &'a str,
    pub fst2_lcd_db_path: &'a str,
    pub fst2_diag_db_path: &'a str,
    pub fst2_key_db_path: &'a str,
    pub bst1_db_path_exist: &'a str,
    pub bst2_db_path_exist: &'a str,
    pub fst1_lcd_db_path_exist: &'a str,
    pub fst1_diag_db_path_exist: &'a str,
    pub fst1_key_db_path_exist: &'a str,
    pub fst2_lcd_db_path_exist: &'a str,
    pub fst2_diag_db_path_exist: &'a str,
    pub fst2_key_db_path_exist: &'a str,
    // base.html
    // all tem variant
    pub tem_language: &'a str,
    pub tem_day: &'a str,
    pub tem_night: &'a str,
    pub tem_previous_day: &'a str,
    pub tem_previous_shift: &'a str,
    pub tem_today: &'a str,
    pub tem_viewing_data: &'a str,
    pub tem_colon: &'a str,
    pub tem_home: &'a str,
    pub tem_quantity_of_pass_fail: &'a str,
    pub tem_yield_of_shift: &'a str,
    pub tem_fail_record_details: &'a str,
    pub tem_query_400_records_of_cell: &'a str,
    pub tem_query_sn_history: &'a str,
    pub tem_query_sn_history_all: &'a str,
    pub tem_key_name_of_bgibest: &'a str,
    pub tem_port_config_of_terminal_server: &'a str,
    pub tem_line_0: &'a str,
    pub tem_line_1: &'a str,
    pub tem_line_2: &'a str,
    pub tem_line_3: &'a str,
    pub tem_submit: &'a str,
    pub tem_local_time: &'a str,
    pub tem_sort_able: &'a str,
}

#[derive(Template)]
#[template(path = "portconfig.html")]
pub struct PortconfigTemplate<'a> {
    pub title: String,
    pub line: String,
    pub hostname: String,
    pub update_time: String,
    pub tem_local_time: &'a str,
    pub tem_language: &'a str,
    pub tem_quantity_of_pass_fail: &'a str,
    pub tem_yield_of_shift: &'a str,
    pub tem_fail_record_details: &'a str,
    pub tem_query_400_records_of_cell: &'a str,
    pub tem_query_sn_history: &'a str,
    pub tem_query_sn_history_all: &'a str,
    pub tem_key_name_of_bgibest: &'a str,
    pub tem_port_config_of_terminal_server: &'a str,
    pub tem_line_1: &'a str,
    pub tem_line_2: &'a str,
    pub tem_line_3: &'a str,
    pub tem_line_4: &'a str,
}

#[derive(Template)]
#[template(path = "keyname.html")]
pub struct KeynameTemplate<'a> {
    pub title: String,
    pub line: String,
    pub hostname: String,
    pub update_time: String,
    pub tem_local_time: &'a str,
    pub tem_language: &'a str,
    pub tem_quantity_of_pass_fail: &'a str,
    pub tem_yield_of_shift: &'a str,
    pub tem_fail_record_details: &'a str,
    pub tem_query_400_records_of_cell: &'a str,
    pub tem_query_sn_history: &'a str,
    pub tem_query_sn_history_all: &'a str,
    pub tem_key_name_of_bgibest: &'a str,
    pub tem_port_config_of_terminal_server: &'a str,
    pub tem_line_1: &'a str,
    pub tem_line_2: &'a str,
    pub tem_line_3: &'a str,
}

#[derive(Template)]
#[template(path = "all_cell_record.html")]
pub struct StationYieldTemplate<'a> {
    pub station_yield: Vec<(&'a str, u16, u16, u16, u16, String)>,
    // base.html
    pub title: String,
    pub line: String,
    pub hostname: String,
    pub update_time: String,
    // all tem variant
    pub tem_language: &'a str,
    pub tem_day: &'a str,
    pub tem_night: &'a str,
    pub tem_previous_day: &'a str,
    pub tem_previous_shift: &'a str,
    pub tem_today: &'a str,
    pub tem_viewing_data: &'a str,
    pub tem_colon: &'a str,
    pub tem_home: &'a str,
    pub tem_quantity_of_pass_fail: &'a str,
    pub tem_yield_of_shift: &'a str,
    pub tem_fail_record_details: &'a str,
    pub tem_query_400_records_of_cell: &'a str,
    pub tem_query_sn_history: &'a str,
    pub tem_query_sn_history_all: &'a str,
    pub tem_key_name_of_bgibest: &'a str,
    pub tem_port_config_of_terminal_server: &'a str,
    pub tem_line_0: &'a str,
    pub tem_line_1: &'a str,
    pub tem_line_2: &'a str,
    pub tem_line_3: &'a str,
    pub tem_submit: &'a str,
    pub tem_local_time: &'a str,
    pub tem_sort_able: &'a str,
}


#[derive(Template)]
#[template(path = "cell_record_fst.html")]
pub struct CellFstTemplate<'a> {
    pub query_count: u16,
    pub cell: &'a str,
    pub res_yield: (u16, u16, u16, u16),
    pub record: Vec<Rcell>,
    // base.html
    pub title: String,
    pub line: String,
    pub hostname: String,
    pub update_time: String,
    // all tem variant
    pub tem_language: &'a str,
    pub tem_day: &'a str,
    pub tem_night: &'a str,
    pub tem_previous_day: &'a str,
    pub tem_previous_shift: &'a str,
    pub tem_today: &'a str,
    pub tem_viewing_data: &'a str,
    pub tem_colon: &'a str,
    pub tem_home: &'a str,
    pub tem_quantity_of_pass_fail: &'a str,
    pub tem_yield_of_shift: &'a str,
    pub tem_fail_record_details: &'a str,
    pub tem_query_400_records_of_cell: &'a str,
    pub tem_query_sn_history: &'a str,
    pub tem_query_sn_history_all: &'a str,
    pub tem_key_name_of_bgibest: &'a str,
    pub tem_port_config_of_terminal_server: &'a str,
    pub tem_line_0: &'a str,
    pub tem_line_1: &'a str,
    pub tem_line_2: &'a str,
    pub tem_line_3: &'a str,
    pub tem_submit: &'a str,
    pub tem_local_time: &'a str,
    pub tem_sort_able: &'a str,
}

#[derive(Template)]
#[template(path = "cell_record_bst.html")]
pub struct CellBstTemplate<'a> {
    pub query_count: u16,
    pub cell: &'a str,
    pub res_yield: (u16, u16, u16, u16),
    pub record: Vec<Rcell>,
    // base.html
    pub title: String,
    pub line: String,
    pub hostname: String,
    pub update_time: String,
    // all tem variant
    pub tem_language: &'a str,
    pub tem_day: &'a str,
    pub tem_night: &'a str,
    pub tem_previous_day: &'a str,
    pub tem_previous_shift: &'a str,
    pub tem_today: &'a str,
    pub tem_viewing_data: &'a str,
    pub tem_colon: &'a str,
    pub tem_home: &'a str,
    pub tem_quantity_of_pass_fail: &'a str,
    pub tem_yield_of_shift: &'a str,
    pub tem_fail_record_details: &'a str,
    pub tem_query_400_records_of_cell: &'a str,
    pub tem_query_sn_history: &'a str,
    pub tem_query_sn_history_all: &'a str,
    pub tem_key_name_of_bgibest: &'a str,
    pub tem_port_config_of_terminal_server: &'a str,
    pub tem_line_0: &'a str,
    pub tem_line_1: &'a str,
    pub tem_line_2: &'a str,
    pub tem_line_3: &'a str,
    pub tem_submit: &'a str,
    pub tem_local_time: &'a str,
    pub tem_sort_able: &'a str,
}

#[derive(Template)]
#[template(path = "fail_detail_fst.html")]
pub struct DetailFstTemplate<'a> {
    pub data_day: &'a str,
    pub data_shift: &'a str,
    pub lcdled: Vec<Rdetail>,
    pub diag: Vec<Rdetail>,
    pub keypad: Vec<Rdetail>,
    // base.html
    pub title: String,
    pub line: String,
    pub hostname: String,
    pub update_time: String,
    // all tem variant
    pub tem_language: &'a str,
    pub tem_day: &'a str,
    pub tem_night: &'a str,
    pub tem_previous_day: &'a str,
    pub tem_previous_shift: &'a str,
    pub tem_today: &'a str,
    pub tem_viewing_data: &'a str,
    pub tem_colon: &'a str,
    pub tem_home: &'a str,
    pub tem_quantity_of_pass_fail: &'a str,
    pub tem_yield_of_shift: &'a str,
    pub tem_fail_record_details: &'a str,
    pub tem_query_400_records_of_cell: &'a str,
    pub tem_query_sn_history: &'a str,
    pub tem_query_sn_history_all: &'a str,
    pub tem_key_name_of_bgibest: &'a str,
    pub tem_port_config_of_terminal_server: &'a str,
    pub tem_line_0: &'a str,
    pub tem_line_1: &'a str,
    pub tem_line_2: &'a str,
    pub tem_line_3: &'a str,
    pub tem_submit: &'a str,
    pub tem_local_time: &'a str,
    pub tem_sort_able: &'a str,
}

#[derive(Template)]
#[template(path = "fail_detail_bst.html")]
pub struct DetailBstTemplate<'a> {
    pub data_day: &'a str,
    pub data_shift: &'a str,
    pub bst: Vec<Rdetail>,
    // base.html
    pub title: String,
    pub line: String,
    pub hostname: String,
    pub update_time: String,
    // all tem variant
    pub tem_language: &'a str,
    pub tem_day: &'a str,
    pub tem_night: &'a str,
    pub tem_previous_day: &'a str,
    pub tem_previous_shift: &'a str,
    pub tem_today: &'a str,
    pub tem_viewing_data: &'a str,
    pub tem_colon: &'a str,
    pub tem_home: &'a str,
    pub tem_quantity_of_pass_fail: &'a str,
    pub tem_yield_of_shift: &'a str,
    pub tem_fail_record_details: &'a str,
    pub tem_query_400_records_of_cell: &'a str,
    pub tem_query_sn_history: &'a str,
    pub tem_query_sn_history_all: &'a str,
    pub tem_key_name_of_bgibest: &'a str,
    pub tem_port_config_of_terminal_server: &'a str,
    pub tem_line_0: &'a str,
    pub tem_line_1: &'a str,
    pub tem_line_2: &'a str,
    pub tem_line_3: &'a str,
    pub tem_submit: &'a str,
    pub tem_local_time: &'a str,
    pub tem_sort_able: &'a str,
}

#[derive(Template)]
#[template(path = "sn_record.html")]
pub struct SnRecordTemplate<'a> {
    pub record: Vec<Rrecord<'a>>,
    // base.html
    pub title: String,
    pub line: String,
    pub hostname: String,
    pub update_time: String,
    // all tem variant
    pub tem_language: &'a str,
    pub tem_day: &'a str,
    pub tem_night: &'a str,
    pub tem_previous_day: &'a str,
    pub tem_previous_shift: &'a str,
    pub tem_today: &'a str,
    pub tem_viewing_data: &'a str,
    pub tem_colon: &'a str,
    pub tem_home: &'a str,
    pub tem_quantity_of_pass_fail: &'a str,
    pub tem_yield_of_shift: &'a str,
    pub tem_fail_record_details: &'a str,
    pub tem_query_400_records_of_cell: &'a str,
    pub tem_query_sn_history: &'a str,
    pub tem_query_sn_history_all: &'a str,
    pub tem_key_name_of_bgibest: &'a str,
    pub tem_port_config_of_terminal_server: &'a str,
    pub tem_line_0: &'a str,
    pub tem_line_1: &'a str,
    pub tem_line_2: &'a str,
    pub tem_line_3: &'a str,
    pub tem_submit: &'a str,
    pub tem_local_time: &'a str,
    pub tem_sort_able: &'a str,
}

#[derive(Template)]
#[template(path = "day_yield_bst.html")]
pub struct YiedlBstTemplate<'a> {
    pub data_day: &'a str,
    pub data_shift: &'a str,
    pub bst: [(&'a str, u16, u16, u16, u16); 13],
    // base.html
    pub title: String,
    pub line: String,
    pub hostname: String,
    pub update_time: String,
    // all tem variant
    pub tem_language: &'a str,
    pub tem_day: &'a str,
    pub tem_night: &'a str,
    pub tem_previous_day: &'a str,
    pub tem_previous_shift: &'a str,
    pub tem_today: &'a str,
    pub tem_viewing_data: &'a str,
    pub tem_colon: &'a str,
    pub tem_home: &'a str,
    pub tem_quantity_of_pass_fail: &'a str,
    pub tem_yield_of_shift: &'a str,
    pub tem_fail_record_details: &'a str,
    pub tem_query_400_records_of_cell: &'a str,
    pub tem_query_sn_history: &'a str,
    pub tem_query_sn_history_all: &'a str,
    pub tem_key_name_of_bgibest: &'a str,
    pub tem_port_config_of_terminal_server: &'a str,
    pub tem_line_0: &'a str,
    pub tem_line_1: &'a str,
    pub tem_line_2: &'a str,
    pub tem_line_3: &'a str,
    pub tem_submit: &'a str,
    pub tem_local_time: &'a str,
    pub tem_sort_able: &'a str,
}


#[derive(Template)]
#[template(path = "day_yield_fst.html")]
pub struct YiedlFstTemplate<'a> {
    pub data_day: &'a str,
    pub data_shift: &'a str,
    pub lcdled: [(&'a str, u16, u16, u16, u16); 13],
    pub diag: [(&'a str, u16, u16, u16, u16); 13],
    pub keypad: [(&'a str, u16, u16, u16, u16); 13],
    // base.html
    pub title: String,
    pub line: String,
    pub hostname: String,
    pub update_time: String,
    // all tem variant
    pub tem_language: &'a str,
    pub tem_day: &'a str,
    pub tem_night: &'a str,
    pub tem_previous_day: &'a str,
    pub tem_previous_shift: &'a str,
    pub tem_today: &'a str,
    pub tem_viewing_data: &'a str,
    pub tem_colon: &'a str,
    pub tem_home: &'a str,
    pub tem_quantity_of_pass_fail: &'a str,
    pub tem_yield_of_shift: &'a str,
    pub tem_fail_record_details: &'a str,
    pub tem_query_400_records_of_cell: &'a str,
    pub tem_query_sn_history: &'a str,
    pub tem_query_sn_history_all: &'a str,
    pub tem_key_name_of_bgibest: &'a str,
    pub tem_port_config_of_terminal_server: &'a str,
    pub tem_line_0: &'a str,
    pub tem_line_1: &'a str,
    pub tem_line_2: &'a str,
    pub tem_line_3: &'a str,
    pub tem_submit: &'a str,
    pub tem_local_time: &'a str,
    pub tem_sort_able: &'a str,
}


#[derive(Template)]
#[template(path = "pf_data_bst.html")]
pub struct PfBstTemplate<'a> {
    pub data_day: &'a str,
    pub data_shift: &'a str,
    pub bst: Vec<Rpf<'a>>,
    // base.html
    pub title: String,
    pub line: String,
    pub hostname: String,
    pub update_time: String,
    // all tem variant
    pub tem_language: &'a str,
    pub tem_day: &'a str,
    pub tem_night: &'a str,
    pub tem_previous_day: &'a str,
    pub tem_previous_shift: &'a str,
    pub tem_today: &'a str,
    pub tem_viewing_data: &'a str,
    pub tem_colon: &'a str,
    pub tem_home: &'a str,
    pub tem_quantity_of_pass_fail: &'a str,
    pub tem_yield_of_shift: &'a str,
    pub tem_fail_record_details: &'a str,
    pub tem_query_400_records_of_cell: &'a str,
    pub tem_query_sn_history: &'a str,
    pub tem_query_sn_history_all: &'a str,
    pub tem_key_name_of_bgibest: &'a str,
    pub tem_port_config_of_terminal_server: &'a str,
    pub tem_line_0: &'a str,
    pub tem_line_1: &'a str,
    pub tem_line_2: &'a str,
    pub tem_line_3: &'a str,
    pub tem_submit: &'a str,
    pub tem_local_time: &'a str,
    pub tem_sort_able: &'a str,
}


#[derive(Template)]
#[template(path = "pf_data_fst.html")]
pub struct PfFstTemplate<'a> {
    pub data_day: &'a str,
    pub data_shift: &'a str,
    pub lcdled: Vec<Rpf<'a>>,
    pub diag: Vec<Rpf<'a>>,
    pub keypad: Vec<Rpf<'a>>,
    // base.html
    pub title: String,
    pub line: String,
    pub hostname: String,
    pub update_time: String,
    // all tem variant
    pub tem_language: &'a str,
    pub tem_day: &'a str,
    pub tem_night: &'a str,
    pub tem_previous_day: &'a str,
    pub tem_previous_shift: &'a str,
    pub tem_today: &'a str,
    pub tem_viewing_data: &'a str,
    pub tem_colon: &'a str,
    pub tem_home: &'a str,
    pub tem_quantity_of_pass_fail: &'a str,
    pub tem_yield_of_shift: &'a str,
    pub tem_fail_record_details: &'a str,
    pub tem_query_400_records_of_cell: &'a str,
    pub tem_query_sn_history: &'a str,
    pub tem_query_sn_history_all: &'a str,
    pub tem_key_name_of_bgibest: &'a str,
    pub tem_port_config_of_terminal_server: &'a str,
    pub tem_line_0: &'a str,
    pub tem_line_1: &'a str,
    pub tem_line_2: &'a str,
    pub tem_line_3: &'a str,
    pub tem_submit: &'a str,
    pub tem_local_time: &'a str,
    pub tem_sort_able: &'a str,
}