use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

#[tokio::main]
pub async fn scrape(link: String, link_vec: &mut Vec<String>) -> Result<()> {
    let res = reqwest::get(link)
        .await?
        .text()
        .await?;

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| link_vec.push(x.to_string()));
        // .for_each(|x| println!("{}", x));
    return Ok(());
}