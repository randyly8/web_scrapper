mod scrape;

fn main()
{
    let args: Vec<String> = std::env::args().collect();
    scrape::scrape(args[1].clone());
}