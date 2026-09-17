use traits::{aggregator::{self, NewsArticle, Summary, Tweet}, bounds::Pair};


fn main() {
    // Definiendo Traits
    let tweet = Tweet {
        username: String::from("jpTweet"),
        content: String::from(
            "en fin, la hipopotamo"
            ),
        reply: false,
        retweeet: false,
    };

    println!("1 nuevo tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Pinguinos ganan el Campeonato Stanley Cup"),
        location: String::from("Pittsburg, PA, EEUU"),
        author: String::from("Pinguino Gonzales"),
        content: String::from(
            "Los Pinguinos de Pittsburg son los mejores una vez más en la NHL"
            ),
    };

    println!("Nuevo articulo: {}", article.summarize());

    // Traits como parámetros
    aggregator::notify(&tweet);

    // Usar Trait bounds para implementar métodos de manera condicional
    let pair_x = Pair::new(7, 6);
    let pair_y = Pair::new('a', 'z');
    let pair = Pair::new(10.0, 10.0);

    pair_x.cmp_display_greater_of_pair();
    pair_y.cmp_display_greater_of_pair(); // Si pair está conformado por campos que implementen Display, pair implementa Display.
    pair.cmp_display_greater_of_pair(); // Como pair implementa Display, tambien implementa to_string.
}