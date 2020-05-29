mod gsc;

fn main() {
    let mut conn = gsc::Connection::new();
    let (hub, sheet_id) = conn.get_hub();
    let reader = gsc::Reader::new(hub, &sheet_id, "TestSheet", 1, 1, 5, 5);
    let data = reader.read();
    let vec_data = data.unwrap();
    
    conn = gsc::Connection::new();
    let (hub, sheet_id) = conn.get_hub();
    let deleter = gsc::Deleter::new(hub, &sheet_id, "TestSheet", 1, 9, 4, 15);
    let _delete_result = deleter.delete();
    
    conn = gsc::Connection::new();
    let (hub, sheet_id) = conn.get_hub();
    let writer = gsc::Writer::new(hub, &sheet_id, "TestSheet", 1, 9, vec_data,
                                        gsc::ValueOption::UserEntered);
    let _write_result = writer.write();
}
