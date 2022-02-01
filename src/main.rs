mod scrape;

#[tokio::main]
async fn main()
{
    // Set up
    let args: Vec<String> = std::env::args().collect();
    let link: String = args[1].clone();
    let mut link_vec: Vec<String> = vec![];

    // Scrape
    scrape::scrape(link, &mut link_vec)
        .await
        .expect("Could not scrape link!");

    // Print list values
    for i in link_vec.iter()
    {
        println!("{}", i);
        // scrape::download_image(i);
    }
}