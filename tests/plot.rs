use inditech::{Frame, EMA, ROC, RSI, SMA};
use rand::{thread_rng, Rng};

fn generate_data() -> Vec<f32> {
    let mut rng = thread_rng();
    let mut v = Vec::with_capacity(256);

    v.push(10.);

    for _ in 1..v.capacity() {
        let item = rng.gen::<f32>();
        v.push((item - 0.5) * 1. + v.last().copied().unwrap_or_default());
    }

    v
}

#[test]
fn plot() {
    let data = generate_data();
    let mut frame = Frame::new();

    frame.add(Box::new(SMA::new(8)));
    frame.add(Box::new(EMA::new(8)));
    frame.add(Box::new(RSI::new(8)));
    frame.add(Box::new(ROC::new(8)));

    let file = std::fs::File::create("data.csv").unwrap();
    let mut wtr = csv::Writer::from_writer(file);

    let mut headers = vec!["data".to_owned()];
    headers.append(&mut frame.names());

    wtr.write_record(&headers).unwrap();
    for i in 0..data.len() {
        let item = *data.get(i).unwrap();
        frame.push(item);

        let mut row = vec![item];
        row.append(&mut frame.last());

        let row_str: Vec<String> = row.into_iter().map(|value| value.to_string()).collect();
        wtr.write_record(row_str).unwrap();
    }

    wtr.flush().unwrap();
}
