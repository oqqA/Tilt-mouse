extern crate simple_excel_writer as excel;
use excel::*;

pub fn write() {
    let mut wb = Workbook::create("/b.xlsx");

    let mut sheet = wb.create_sheet("Sheet");
    wb.write_sheet(&mut sheet, |sheet_writer| {
        let sw = sheet_writer;
        sw.append_row(row!["Name", "Title","Success","Remark"])?;
        sw.append_row(row!["Amy", "Manager", true])
    }).expect("write excel error!");

    wb.close().expect("close excel error!");
}