#[derive(Serialize, Deserialize)]
struct Message<'r> {
   contents: &'r str,
}

#[put("/<id>", data = "<msg>")]
fn update(db: &Db, id: Id, msg: Json<Message<'_>>) -> Value {
    if db.contains_key(&id) {
        db.insert(id, msg.contents);
        json!({ "status": "ok" })
    } else {
        json!({ "status": "error" })
    }
}