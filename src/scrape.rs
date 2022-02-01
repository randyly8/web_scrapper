use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

pub async fn scrape(link: String, link_vec: &mut Vec<String>) -> Result<()> {
    println!("Point 1"); // Ok
    let res = reqwest::get(&link)
        .await?
        .text()
        .await?;

    println!("Point 2");
    Document::from(res.as_str())
        .find(Name("img"))
        .filter_map(|n| n.attr("src"))
        .for_each(|x| link_vec.push(x.to_string()));
        // .for_each(|x| download_image(&x.to_string()));
        // .for_each(|x| println!("{}", x));
    println!("Point 4");
    return Ok(());
}

pub fn download_image(link: &str)
{
    println!("Point 3");
    let mut file = std::fs::File::create(link).unwrap();
    reqwest::blocking::get(link)
        .unwrap()
        .copy_to(&mut file)
        .unwrap();
}