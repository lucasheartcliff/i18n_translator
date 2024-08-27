# i18n Translator

The **i18n Translator** is a command-line tool written in Rust that automates the translation of i18n JSON files. It reads an English-language JSON file and translates the content into a list of specified languages using an external translation API.

## Features

- **Automated Translation:** Translate key-value pairs from English to multiple languages.
- **Configurable Languages:** Easily specify which languages you want to translate to.
- **JSON Handling:** Reads and processes standard i18n JSON files.

## Prerequisites

- **Rust**: Ensure that Rust is installed on your system. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/learn/get-started).

## Installation

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/lucasheatcliff/i18n_translator.git
   cd i18n_translator
    ```

2. **Build the Project:**

   ```bash
   cargo build --release
   ```

3. **Run the Project:**

   ```bash
   cargo run --release
   ```

## Usage

1. **Prepare your i18n JSON file:**

   Ensure your i18n JSON file is structured like this:

   ```json
   {
     "en": {
       "hello": "Hello",
       "welcome": "Welcome",
       "goodbye": "Goodbye"
     }
   }
   ```

   Save this file as `i18n.json` in the root directory of the project.

2. **Specify Target Languages:**

   In the `src/main.rs` file, edit the `languages` vector to include the ISO 639-1 codes of the languages you want to translate to:

   ```rust
   let languages = vec!["es", "fr", "de"]; // Spanish, French, German
   ```

3. **Set Up Your API Key:**

   Replace the placeholder in the `translate_text` function with your actual API key and the translation API URL:

   ```rust
   let api_key = "your_api_key_here"; // Replace with your actual API key
   let api_url = "https://api.yourtranslationprovider.com/translate"; // Replace with your actual API URL
   ```

4. **Run the Program:**

   ```bash
   cargo run --release
   ```

   The program will read the `i18n.json` file, call the translation API, and print the translated results to the console.

5. **Save the Translations (Optional):**

   You can modify the code to save the translations back into a JSON file if needed.

## Example Output

After running the program, the output will look something like this:

```plaintext
Key: hello
  es: Hola
  fr: Bonjour
  de: Hallo
Key: welcome
  es: Bienvenido
  fr: Bienvenue
  de: Willkommen
Key: goodbye
  es: Adiós
  fr: Au revoir
  de: Auf Wiedersehen
```

## Contributing

Contributions are welcome! If you have suggestions for improvements or new features, feel free to open an issue or submit a pull request.

