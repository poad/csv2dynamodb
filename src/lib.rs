use std::collections::HashMap;
use std::error::Error as Err;
use std::fs::File;
use csv::Reader;

pub struct Csv {
    pub headers: Vec<String>,
    pub records: Vec<HashMap<String, String>>
}

pub fn read_csv(file_path: &str) -> Result<Csv, Box<dyn Err>> {
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(file);
    let mut headers = Vec::new();

    reader.headers()?.iter()
        .for_each(|header| headers.push(String::from(header)));

    let columns = headers.len();
    let records = reader.records()
        .map(|result| {
            let r = result.unwrap();
            let mut record = HashMap::new();
            for i in (0..).take(columns) {
                let value = r.get(i).unwrap();
                record.insert(String::from(&headers[i]), String::from(value));
            }
            record
        })
        .collect();
    Ok(Csv{headers, records})
}

#[cfg(test)]
mod tests {

}
