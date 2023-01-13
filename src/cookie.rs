pub enum Cookie {
    CookieString(String),
    CookieParsed(String, String, String, String, String)
}