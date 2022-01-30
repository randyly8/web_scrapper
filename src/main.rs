mod scrape;

fn main()
{
    let args: Vec<String> = std::env::args().collect();
    let link: String = args[1].clone();
    let depth: u8 = args[2].parse::<u8>().unwrap();
    let mut link_vec: Vec<String> = vec![];

    scrape::scrape(link, &mut link_vec);

    // Print link_vec values
    for i in link_vec.iter()
    {
        println!("{}", i);
    }

    // Do something with the data?
}