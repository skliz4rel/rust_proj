fn main() {
    //This is a normal string
    let data_string = "{

    \"name\": \"John Doe\",

    \"age\": 43,

    \"phones\": [

      \"+44 1234567\",

      \"+44 2345678\"

    ]

  }";

    //Raw string literal

    let data_raw_string = r#"{

    "name": "John Doe",

    "age": 43,

    "phones": [

      "+44 1234567",

      "+44 2345678"

    ]

  }"#;

    println!("This is a normal string {}", data_string);

    println!("This is a raw string literal {}", data_raw_string);
}
