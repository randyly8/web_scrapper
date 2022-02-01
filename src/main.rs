mod scrape;

fn main()
{
    let args: Vec<String> = std::env::args().collect();
    let link: String = args[1].clone();
    let mut link_vec: Vec<String> = vec![];

    scrape::scrape(link, &mut link_vec);

    // Download Images
    for i in link_vec.iter()
    {
        println!("{}", i);
        scrape::download_image(i);
    }
}

