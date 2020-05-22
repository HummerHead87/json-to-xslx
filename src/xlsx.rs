use std::collections::HashMap;
use xlsxwriter::*;

pub fn write_to_xlsx (contents: &HashMap<Vec<String>, String>, output: &str, separator: &str) {
    let wb = Workbook::new(output);
    let mut sheet = wb.add_worksheet(Some("Dictionary")).unwrap();

    let mut keys: Vec<_> = contents.keys().collect();
    keys.sort();

    let mut row = 0;
    for key in keys.iter() {
        let val = contents.get(*key).unwrap();
        let key = key.join(separator);
        
        sheet.write_string(row, 0, &key, None)
            .expect("Error write in excel file");
        sheet.write_string(row, 1, val, None)
            .expect("Error write in excel file");

        row += 1;
    }
    
    wb.close().expect("close excel error!");
}
