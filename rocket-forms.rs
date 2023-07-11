#[derive(FromForm)]
struct Task<'r> {
   #[field(validate = len(1..))]
   description: &'r str,
   completed: bool
}

#[post("/", data = "<task>")]
fn new(task: Form<Task<'_>>) -> Flash<Redirect> {
    Flash::success(Redirect::to(uri!(home)), "Task added.")
}