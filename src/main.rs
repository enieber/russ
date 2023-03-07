use std::*;

type Link = String;

//TODO: defined by reference in rss doc: https://www.rssboard.org/rss-language-codes
enum Lang {
    en_us,
    pt_br,
}

//example: geo@herald.com (George Matesky)
type Person = String;

type Date = String;

type Category = String;

/*enum Protocol {
  http:  "http-post",
  xml: "xml-rpc",
  soap: "soap 1.1",
}
*/
enum Protocol {
    Http,
    Xml,
    Soap,
}

struct Cloud {
    domain: String,
    port: i32,
    path: String,
    register_procedure: String,
    protocol: Protocol,
}

//TODO: implement the validator url with GIF/JPEG/PNG
type LinkImage = Link;

struct Image {
    url: LinkImage,
    title: String,
    link: Link,
    //TODO: define maximus value with 144 and default value 88
    width: Option<i32>,
    //TODO: define maximus value with 400 and default value 31
    height: Option<i32>,
    description: Option<String>,
}

struct TextInput {
    title: String,
    description: String,
    name: String,
    link: Link,
}

enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

struct Channel {
    title: String,
    link: Link,
    description: String,
    language: Option<Lang>,
    copyright: Option<String>,
    managing_editor: Option<Person>,
    web_master: Option<Person>,
    pub_date: Option<Date>,
    last_build_date: Option<Category>,
    generator: Option<String>,
    docs: Option<Link>,
    cloud: Option<Cloud>,
    ttl: Option<time::Duration>,
    image: Option<Image>,
    //TODO: deprecated to W3C: https://www.w3.org/2009/08/pics_superseded.html
    rating: Option<String>,
    text_input: Option<TextInput>,
    //TODO: Define Range start: 0 end 23
    skip_hours: Option<ops::Range<i32>>,
    skip_days: Option<Days>,
}

fn main() {
    println!("Hello, world!");
}
