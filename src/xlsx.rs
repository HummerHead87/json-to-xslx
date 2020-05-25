use std::collections::HashMap;
// use std::cmp::Ordering;
use xlsxwriter::*;
use crate::helpers::Config;

pub fn write_to_xlsx (contents: &HashMap<Vec<String>, String>, config: &Config) {
    let wb = Workbook::new(&config.output);
    let mut sheet = wb.add_worksheet(Some("Dictionary")).unwrap();

    let mut keys: Vec<_> = contents.keys().collect();
    keys.sort();
    // keys.sort_by(|a, b| {
    //     if a.len() < b.len() {
    //         Ordering::Less
    //     } else if a.len() > b.len() {
    //         Ordering::Greater
    //     } else {
    //         a.cmp(b)
    //     }
    // });
    let error_message = "Error write in excel file";

    sheet.write_string(0, 0, "key", None)
        .expect(error_message);
    sheet.write_string(0, 1, &config.language, None)
        .expect(error_message);

    let mut row = 1;
    for key in keys.iter() {
        let val = contents.get(*key).unwrap();
        let key = key.join(&config.separator);
        
        sheet.write_string(row, 0, &key, None)
            .expect(error_message);
        sheet.write_string(row, 1, val, None)
            .expect(error_message);

        row += 1;
    }
    
    wb.close().expect("close excel error!");
}
