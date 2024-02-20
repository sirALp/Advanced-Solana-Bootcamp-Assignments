fn main () {
    let book1 = Book{
        title: "Tom Sawyer's Adventures #1".to_string(),
        author: "Mark Twain".to_string(),
        page_count:68
    };

    let book2 = Book {
        title:"Tom Sawyer's Adventures #2".to_string(),
        author: "Mark Twain".to_string(),
        page_count:75
    };
    

    let mag1 = Magazine {
        title: "GQ".to_string(),
        issue: 6,
        topic: "Fashion!".to_string()
    };

    let mag2 = Magazine {
        title: "Forbes".to_string(),
        issue: 25,
        topic: "Rich ppl :3".to_string()
    };

    let publications = vec![
        Publication::Book(book1),
        Publication::Book(book2),
        Publication::Magazine(mag1),
        Publication::Magazine(mag2)
    ];

    print_publications(publications);

}

enum Publication {
    Book(Book),
    Magazine(Magazine)
}

struct Book {
    title:String,
    author:String,
    page_count:u32
}

struct Magazine {
    title:String,
    issue:u32,
    topic:String
}

fn print_publications(publications: Vec<Publication>) {
    for each in publications {
        match each {
            Publication::Book(ref book) =>{ 
                println!("Book's Title: {},Author: {},Page Count:{}"
                ,book.title,book.author,book.page_count);
            }
            Publication::Magazine(ref magazine) =>{ 
                println!("Magazine's Title: {},Issue: {},Topic:{}"
                ,magazine.title,magazine.issue,magazine.topic);
            }
        }
    }
}

