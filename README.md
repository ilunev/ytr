# ytr
Yandex.Translate API wrapper for Rust
- [Documentation](https://docs.rs/ytr)
- [crates.io](https://crates.io/crates/ytr)
- [About Yandex.Translate API](https://tech.yandex.ru/translate/) (in Russian)

## Usage example
```rust
let key = String::from("my-api-key");
let api = ytr::ApiClient::new(key);

let result = api.translate("Hello!", "ru")   // required parameters
    .format("plain")                         // optional parameter
    .get();                                  // execute the request
 
match result {
    Ok(response) => {
        println!("{}", response.text);       // prints "Привет!"
        println!("{}", response.lang);       // prints "en-ru"
    },
     
    Err(error) => {
        println!(
            "An error has occured: {:?}",
            error
        );
    },
};
```

## License
Licensed under [MIT](LICENSE) license

## Contribution
You are free to propose changes and contribute. Any input is welcome =)
