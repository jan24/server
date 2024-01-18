use rusqlite::{Connection, Rows};
use std::error::Error;
use std::io::Error as IoError;
use std::io::ErrorKind::NotFound;
use crate::analysis::config::CONFIG;
use crate::analysis::mytime;

static LINES: [&str; 4] = ["bst1", "bst2", "fst1", "fst2"];
static LCDLED_CELL: [&str; 6] = ["CELL_81", "CELL_82", "CELL_83", "CELL_84", "CELL_85", "CELL_86"];
static DIAG_CELL: [&str; 8] = ["CELL_53", "CELL_55", "CELL_57", "CELL_59", "CELL_61", "CELL_63", "CELL_65", "CELL_67"];
static KEYPAD_CELL: [&str; 6] = ["CELL_69", "CELL_71", "CELL_73", "CELL_75", "CELL_77", "CELL_79"];
static AP3_CELL: [&str; 0] = [];

static BST_CELL: [&str; 8] = [
    "BST_01:DUT_01", "BST_01:DUT_02", "BST_01:DUT_03", "BST_01:DUT_04",
    "BST_01:DUT_05", "BST_01:DUT_06", "BST_01:DUT_07", "BST_01:DUT_08", ];

pub fn cell2station(cell: &str) -> Option<&str> {
    if LCDLED_CELL.contains(&cell) {
        Some("LCDLED")
    } else if DIAG_CELL.contains(&cell) {
        Some("DIAG")
    } else if KEYPAD_CELL.contains(&cell) {
        Some("KEYPAD")
    } else if BST_CELL.contains(&cell) {
        Some("BST")
    } else {
        None
    }
}

fn get_index(ts_per_hour: [(u32, u32); 12], ts: u32) -> usize {
    for (i, x) in ts_per_hour.iter().enumerate() {
        if x.0 <= ts && ts < x.1 {
            return i;
        }
    }
    unreachable!("ts index");
}

fn get_cells(station: &str) -> Vec<&str> {
    match station {
        "BST" => Vec::from(BST_CELL),
        "LCDLED" => Vec::from(LCDLED_CELL),
        "DIAG" => Vec::from(DIAG_CELL),
        "KEYPAD" => Vec::from(KEYPAD_CELL),
        _ => Vec::from(AP3_CELL)
    }
}

fn get_cell_index(cell: &str, station: &str) -> usize {
    let cells = get_cells(station);
    assert!(cells.contains(&cell));
    for (i, x) in cells.iter().enumerate() {
        if x == &cell {
            return i;
        }
    }
    unreachable!("cell index");
}

type Rstation<'a> = (&'a str, u16, u16, u16, u16, String);

/// return ("CELL_85", 399, 348, 51, 2, "12.8 %")
fn rows_to_station_yield<'a>(cell_name: &'a str, mut rows: Rows) -> Rstation<'a> {
    let mut res = (cell_name, 0, 0, 0, 0, "".to_string());
    while let Some(row) = rows.next().unwrap() {
        let (r, q) = (row.get::<_, String>(0).unwrap(), row.get::<_, u16>(1).unwrap());
        match r.as_str() {
            "S" => { res.1 = q; }
            "P" => { res.2 = q; }
            "F" => { res.3 = q; }
            "U" => { res.4 = q; }
            _ => ()
        }
    }
    if res.1 != 0 && res.3 != 0 {
        res.5 = format!("{:.1} %", 100.0 * res.3 as f32 / res.1 as f32);
    }
    res
}

pub fn query_station_yield<'a>(line: &str, station: &str, count: u16) -> Result<Vec<Rstation<'a>>, Box<dyn Error>> {
    assert!(LINES.contains(&line));
    let _line = match line {
        "fst1" => "1",
        "fst2" => "2",
        _ => "1"
    };
    // let cells = get_cells(station); todo, why not ?
    let cells = match station {
        "BST" => Vec::from(BST_CELL),
        "LCDLED" => Vec::from(LCDLED_CELL),
        "DIAG" => Vec::from(DIAG_CELL),
        "KEYPAD" => Vec::from(KEYPAD_CELL),
        _ => Vec::from(AP3_CELL)
    };
    let db_path = CONFIG.get().unwrap().get_db(line, station).unwrap();
    if !db_path.exists() {
        eprintln!("Error: for {line} {station}, db_path {db_path:?} not exist!");
        return Err(Box::new(IoError::new(NotFound, "db file not found")));
    }
    let c = Connection::open(db_path)?;
    let mut v = Vec::new();
    for cell in cells {
        let cell_name = if line.contains("fst") {
            format!("Bgibest Auto FST {_line}|PCBINT|{station}-{_line}|{cell}")
        } else {
            format!("UCEBU Automatic BST New|PCBDG|{station}-01|{cell}")
        };
        let mut stmt = c.prepare("select result,count(result) from
                                       (select result from tst_record where cell=?1 order by id desc limit 0, ?2)
                                        group by result")?;
        let rows = stmt.query(rusqlite::params![cell_name, count * 2])?;
        let res = rows_to_station_yield(cell, rows);
        v.push(res);
    }
    Ok(v)
}

pub type Rcell = (u16, String, String, String, String, String, String, String, String);
pub type RcellYield = ((u16, u16, u16, u16), Vec<Rcell>);

fn row_to_cell(mut rows: Rows) -> RcellYield {
    let (mut s, mut p, mut f, mut u) = (0, 0, 0, 0);
    let mut seq = 0u16;
    let mut fail_data = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        seq += 1;
        // let id = row.get::<_, u32>(0).unwrap();
        let beijing_str = row.get::<_, String>(1).unwrap();
        let _sn = row.get::<_, String>(2).unwrap();
        let sn = if _sn == "FCH11111111" || _sn == "FCH12345678" { "".to_string() } else { _sn };
        let pid = row.get::<_, String>(3).unwrap();
        let pn = row.get::<_, String>(4).unwrap();
        let result = row.get::<_, String>(5).unwrap();
        match result.as_str() {
            "S" => { s += 1; }
            "P" => { p += 1; }
            "F" => { f += 1; }
            "U" => { u += 1; }
            _ => ()
        }
        //"UCEBU Automatic BST New|PCBDG|BST-01|BST_01:DUT_02"
        let _cell = row.get::<_, String>(6).unwrap();
        let sp: Vec<&str> = _cell.split('|').collect();
        let cell = sp[sp.len() - 1].to_string();
        let msg = row.get::<_, String>(7).unwrap();
        let msg_detail = row.get::<_, String>(8).unwrap();
        if result == "F" || result == "U" {
            fail_data.push((seq, beijing_str, sn, pid, pn, result, cell, msg, msg_detail));
        }
    }
    ((s, p, f, u), fail_data)
}


pub fn query_cell(line: &str, cell: &str, count: u16) -> Result<RcellYield, Box<dyn Error>> {
    assert!(LINES.contains(&line));
    let _line = match line {
        "fst1" => "1",
        "fst2" => "2",
        _ => "1"
    };
    let station = cell2station(cell).unwrap();
    let db_path = CONFIG.get().unwrap().get_db(line, station).unwrap();
    if !db_path.exists() {
        eprintln!("Error: for {line} {station}, db_path {db_path:?} not exist!");
        return Err(Box::new(IoError::new(NotFound, "db file not found")));
    }
    let c = Connection::open(db_path)?;
    let cell_name = if line.contains("fst") {
        format!("Bgibest Auto FST {_line}|PCBINT|{station}-{_line}|{cell}")
    } else {
        format!("UCEBU Automatic BST New|PCBDG|{station}-01|{cell}")
    };
    let mut stmt = c.prepare("select id,beijing_str,sn,pid,pn,result,cell,msg,msg_detail from tst_record '
    'where cell=?1 order by id desc limit 0, ?2")?;
    let rows = stmt.query(rusqlite::params![cell_name, count * 2])?;
    let (res_yield, fail_data) = row_to_cell(rows);
    Ok((res_yield, fail_data))
}

pub type Rdetail = (String, String, String, String, String, String, String, String);

fn row_to_detail(mut rows: Rows) -> Vec<Rdetail> {
    let mut fail_data = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let beijing_str = row.get::<_, String>(1).unwrap();
        let _sn = row.get::<_, String>(2).unwrap();
        let sn = if _sn == "FCH11111111" || _sn == "FCH12345678" { "".to_string() } else { _sn };
        let pid = row.get::<_, String>(3).unwrap();
        let pn = row.get::<_, String>(4).unwrap();
        let result = row.get::<_, String>(5).unwrap();
        //UCEBU Automatic BST New|PCBDG|BST-01|BST_01:DUT_02 -> BST_01:DUT_02
        let _cell = row.get::<_, String>(6).unwrap();
        let sp: Vec<&str> = _cell.split('|').collect();
        let cell = sp[sp.len() - 1].to_string();
        let msg = row.get::<_, String>(7).unwrap();
        let msg_detail = row.get::<_, String>(8).unwrap();
        fail_data.push((beijing_str, sn, pid, pn, result, cell, msg, msg_detail));
    }
    fail_data
}

pub fn fail_detail(line: &str, station: &str, dateshift: &mytime::DateShift) -> Result<Vec<Rdetail>, Box<dyn Error>> {
    let (start_ts, end_ts) = mytime::start_end_of_shift(dateshift);
    let db_path = CONFIG.get().unwrap().get_db(line, station).unwrap();
    if !db_path.exists() {
        eprintln!("Error: for {line} {station}, db_path {db_path:?} not exist!");
        return Err(Box::new(IoError::new(NotFound, "db file not found")));
    }
    let c = Connection::open(db_path)?;
    let mut stmt = c.prepare(r#"select id,beijing_str,sn,pid,pn,result,cell,msg,msg_detail from tst_record
                             where (result="F" or result="U") and ?1<=time_int and time_int<?2"#)?;
    let rows = stmt.query([start_ts, end_ts])?;
    Ok(row_to_detail(rows))
}

fn row_to_yield<'a>(mut rows: Rows, ts_per_hour: [(u32, u32); 12], hours_str: &'a [String]) -> [(&'a str, u16, u16, u16, u16); 13] {
    let mut z = [("", 0, 0, 0, 0); 13];
    for i in 0..13 {
        z[i].0 = hours_str[i].as_str();
    }
    while let Some(row) = rows.next().unwrap() {
        let (res, ts) = (row.get::<_, String>(0).unwrap(), row.get::<_, f32>(1).unwrap() as u32);
        let i = get_index(ts_per_hour, ts);
        match res.as_str() {
            "S" => {
                z[i].1 += 1;
                z[12].1 += 1
            }
            "P" => {
                z[i].2 += 1;
                z[12].2 += 1
            }
            "F" => {
                z[i].3 += 1;
                z[12].3 += 1
            }
            "U" => {
                z[i].4 += 1;
                z[12].4 += 1
            }
            _ => ()
        }
    }
    z
}

type RdayYield<'a> = [(&'a str, u16, u16, u16, u16); 13];

pub fn day_yield<'a>(line: &str, station: &str, ts_per_hour: [(u32, u32); 12], hours_str: &'a [String])
                     -> Result<RdayYield<'a>, Box<dyn Error>> {
    let (start_ts, end_ts) = (ts_per_hour[0].0, ts_per_hour[11].1);
    let db_path = CONFIG.get().unwrap().get_db(line, station).unwrap();
    if !db_path.exists() {
        eprintln!("Error: for {line} {station}, db_path {db_path:?} not exist!");
        return Err(Box::new(IoError::new(NotFound, "db file not found")));
    }
    let c = Connection::open(db_path)?;
    let mut stmt = c.prepare("select result,time_int from tst_record where ?1<=time_int and time_int<?2")?;
    let rows = stmt.query([start_ts, end_ts])?;
    Ok(row_to_yield(rows, ts_per_hour, hours_str))
}

pub type Rpf<'a> = (&'a str, String, String, String, String, String, String, String, String, String);


fn rpft_default() -> Vec<[(u16, u16); 9]> {
    let mut v = Vec::new();
    for _ in 0..13 {
        v.push([(0, 0); 9]);
    }
    v
}

fn array2str(array: Vec<[(u16, u16); 9]>) -> Vec<[String; 9]> {
    let mut v: Vec<[String; 9]> = Vec::new();
    for _ in 0..13 {
        v.push(["".to_string(), "".to_string(), "".to_string(), "".to_string(),
            "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()]);
    }
    for i in 0..13 {
        for j in 0..9 {
            v[i][j] = {
                let (l, r) = (array[i][j].0, array[i][j].1);
                if l == 0 && r == 0 {
                    "".to_string()
                } else if l != 0 && r == 0 {
                    format!("{l}")
                } else if l == 0 && r != 0 {
                    format!(" | {r}")
                } else {
                    format!("{l} | {r}")
                }
            }
        }
    }
    v
}

fn array2rfp(array_str: Vec<[String; 9]>, hours_str: &[String]) -> Vec<Rpf> {
    let mut r = Vec::new();
    for i in 0..13 {
        let x: Rpf = (hours_str[i].as_str(), array_str[i][0].to_owned(), array_str[i][1].to_owned(), array_str[i][2].to_owned(),
                      array_str[i][3].to_owned(), array_str[i][4].to_owned(), array_str[i][5].to_owned(),
                      array_str[i][6].to_owned(), array_str[i][7].to_owned(), array_str[i][8].to_owned());
        r.push(x);
    }
    r
}


pub fn row_to_pf<'a>(mut rows: Rows, station: &str, ts_per_hour: [(u32, u32); 12], hours_str: &'a [String])
                     -> Vec<Rpf<'a>> {
    let mut v = rpft_default();
    while let Some(row) = rows.next().unwrap() {
        let res = row.get::<_, String>(0).unwrap();
        let ts = row.get::<_, f32>(1).unwrap() as u32;
        let _cell = row.get::<_, String>(2).unwrap();
        let sp: Vec<&str> = _cell.split('|').collect();
        let cell = sp[sp.len() - 1].to_string();
        let i = get_index(ts_per_hour, ts);
        let j = get_cell_index(&cell, station);
        match res.as_str() {
            "P" => {
                v[i][j + 1].0 += 1;
                v[i][0].0 += 1;
                v[12][j + 1].0 += 1;
                v[12][0].0 += 1;
            }
            "F" => {
                v[i][j + 1].1 += 1;
                v[i][0].1 += 1;
                v[12][j + 1].1 += 1;
                v[12][0].1 += 1;
            }
            _ => ()
        }
    }
    let array_str = array2str(v);
    array2rfp(array_str, hours_str)
}

pub fn pf_data<'a>(line: &str, station: &str, ts_per_hour: [(u32, u32); 12], hours_str: &'a [String])
                   -> Result<Vec<Rpf<'a>>, Box<dyn Error>> {
    let (start_ts, end_ts) = (ts_per_hour[0].0, ts_per_hour[11].1);
    let db_path = CONFIG.get().unwrap().get_db(line, station).unwrap();
    if !db_path.exists() {
        eprintln!("Error: for {line} {station}, db_path {db_path:?} not exist!");
        return Err(Box::new(IoError::new(NotFound, "db file not found")));
    }
    let c = Connection::open(db_path)?;
    let mut stmt = c.prepare("select result,time_int,cell from tst_record where ?1<=time_int and time_int<?2")?;
    let rows = stmt.query([start_ts, end_ts])?;
    Ok(row_to_pf(rows, station, ts_per_hour, hours_str))
}

pub type Rrecord<'a> = (String, String, String, String, &'a str, String, &'a str, String, String, String);

fn row_to_rec<'a>(mut rows: Rows, area: &'a str, hostname: &'a str) -> Vec<Rrecord<'a>> {
    let mut rec_data = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let beijing_str = row.get::<_, String>(0).unwrap();
        let sn = row.get::<_, String>(1).unwrap();
        let pid = row.get::<_, String>(2).unwrap();
        let pn = row.get::<_, String>(3).unwrap();
        let result = row.get::<_, String>(4).unwrap();
        //UCEBU Automatic BST New|PCBDG|BST-01|BST_01:DUT_02 -> BST_01:DUT_02
        let _cell = row.get::<_, String>(5).unwrap();
        let sp: Vec<&str> = _cell.split('|').collect();
        let cell = sp[sp.len() - 1].to_string();
        let msg = row.get::<_, String>(6).unwrap();
        let msg_detail = row.get::<_, String>(7).unwrap();
        rec_data.push((beijing_str, sn, pid, pn, area, result, hostname, cell, msg, msg_detail));
    }
    rec_data
}

pub fn sn_record<'a>(sn: String) -> Result<Vec<Rrecord<'a>>, Box<dyn Error>> {
    let mut v = Vec::new();
    let all_db_detail = CONFIG.get().unwrap().get_all_db();
    for db_detail in all_db_detail.iter() {
        let (area, hostname, db_path) = db_detail;
        if !db_path.exists() {
            eprintln!("Error: query sn_record for {area} {hostname} db_path {db_path:?} not exist!");
        } else {
            let c = Connection::open(db_path)?;
            let mut stmt = c.prepare("select beijing_str,sn,pid,pn,result,cell,msg,msg_detail from tst_record where sn=?1")?;
            let rows = stmt.query([&sn])?;
            let mut r = row_to_rec(rows, area, hostname);
            v.append(&mut r);
        }
    }
    Ok(v)
}
