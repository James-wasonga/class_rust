pub fn print_occupation(name: &str) -> Option<&str> {
  match name{
    "programmer" => Some("Hello fellow Programmer"),
    "mathematician" => Some("Math is great"),
    _=> None

  }
}

pub fn print_number(name: &str) -> Option<i32>{
    match name{
        "kenn" => Some(200),
        "James" => Some(300),
        _=>None
    }

}
