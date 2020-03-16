use std::fs::File;
use std::io::{BufReader,BufRead};
// datatype and inside are not always available
#[derive(Debug)]
pub struct CTagsRow{
    pub key: String,
    pub path: String,
    pub value: String,
    pub datatype: Option<String>,
    pub inside: Option<String>,
}


impl CTagsRow {
    pub fn fill_data(v: Vec<&str>) -> Option<CTagsRow> {
        println!("Length: \t{}", v.len());
        if v.len() == 5 {
            Some(CTagsRow {
                key: v[0].to_string(),
                path: v[1].to_string(),
                value: v[2].to_string(),
                datatype: Some(v[3].to_string()),
                inside: Some(v[4].to_string()),
            })
        }
        else if v.len() == 4 {
            Some(CTagsRow {
            key: v[0].to_string(),
            path: v[1].to_string(),
            value: v[2].to_string(),
            datatype: Some(v[3].to_string()),
            inside: None,
        })
        }
        else if v.len() == 3 {
            Some(CTagsRow {
            key: v[0].to_string(),
            path: v[1].to_string(),
            value: v[2].to_string(),
            datatype: None,
            inside: None,
        })
    }
    else {
        println!("Something wrong happened here");
        Some(CTagsRow {
        key: v[0].to_string(),
        path: v[1].to_string(),
        value: v[2].to_string(),
        datatype: None,
        inside: None,
    })
    }

    }
}
pub fn read() {
    // Local first
    let file = File::open("tags").expect("File does not exist");
    let reader = BufReader::new(file);

let mut elements = Vec::<CTagsRow>::new();

    for line in reader.lines() {
            let result = line.unwrap();
            let v: Vec<&str> = result.rsplit('\t').collect();
            let my_row = CTagsRow::fill_data(v).expect("Broken at unpacking rows");
            elements.push(my_row);
    }
    println!("{:?}",elements);

}
