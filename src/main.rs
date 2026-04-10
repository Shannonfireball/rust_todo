use dioxus::prelude::*;
use rusqlite::Connection;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {

    // let connection = Connection::open( "./data.db3").unwrap();
    // connection.execute("CREATE TABLE app ( id INTEGER PRIMARY KEY, name TEXT NOT NULL )", ()).unwrap();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Main {}

    }
}

#[derive(Debug, Clone, PartialEq)]
struct Item {
    id: u32,
    name: String,
}

#[component]
pub fn Main() -> Element {
    let mut item = use_signal(|| { String::new() });
    let mut items: Signal<Vec<Item>> = use_signal(Vec::<Item>::new);

    let mut connection = use_signal(|| Connection::open( "./data.db3").unwrap() );

    
    
    
    let mut add_item = move | item: String | {
        connection.write().execute("INSERT INTO app (name) VALUES(?1)", [&item]).unwrap();
    };
    
    let mut delete_item = move | item: Item | {
        connection.write().execute("DELETE FROM app WHERE id=?1", [&item.id]).unwrap();
    };

    use_effect( move || {

        items.write().clear();

        let read = connection.read();
        
        let mut fetch_items_statement = read.prepare("SELECT * FROM app").unwrap();
        
        let rows = fetch_items_statement.query_map((), | row|   {
            println!("{:?}",row);
            Ok(Item {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap()
            })
        } ).unwrap();
    
        for row in rows {
            let row_item = row.unwrap();
            items.write().push(row_item)
        }

    });



    rsx! {
        div { 
            class: "header",
            input { 
                type: "text", 
                class: "",
                oninput: move |event| {
                    item.set(event.value());
                },
                onkeypress: move |event| {
                    if event.code().to_string() == "Enter".to_string() {
                        // println!("{:?}", item);
                        // items.write().push(item());
                        // println!("{:?}", items);

                        add_item(item())

                    }
                }
            }
        }

        div {
            for item in items.iter() {
                ItemElement { item: item.clone(), callback: delete_item  }
            }
        }
    }
}


#[component]

fn ItemElement(item: Item, callback: Callback<Item> ) -> Element {
    rsx! {
        div {
            class: "item",
            label { {item.name.clone()} }
            button { 
                onclick: move |_event| {
                    callback(item.clone())
                },
                "delete",
            }
        }
    }
}

