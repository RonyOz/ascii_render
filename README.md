# ascii_render

This project is a web server that renders text in ASCII and animates it in the console.  It's designed to be used with `curl` to display the animation.

## Usage

1.  **Clone the repository:**

    ```bash
    git clone [https://github.com/RonyOz/ascii_render.git](https://github.com/RonyOz/ascii_render.git)
    ```

2.  **Install the dependencies:**

    ```bash
    # Make sure you have Rust and Cargo installed
    cd ascii_render
    cargo build
    ```

3.  **Run the server:**

    ```bash
    cargo run
    ```

4.  **Use `curl` to request the animation:**

    ```bash
    curl [http://127.0.0.1:3030/your-text-here](http://127.0.0.1:3030/your-text-here)
    ```
    Replace `your-text-here` with the text you want to animate.

## Features

*   Renders text in ASCII.
*   Animates the text in the console with a scrolling effect.
*   Utilizes Server-Sent Events (SSE) for efficient streaming of animation frames.

## Future Enhancements

*   **Color Support:** Introduce color to the ASCII animations.
*   **Error Handling:** Implement more robust error handling for API requests and edge cases.
*   **Performance Optimization:** Fine-tune animation speed and resource usage for optimal performance.
*   **Server support:** Deploy a server to render in any terminal with `curl`.

## Contributing

Contributions are welcome! Feel free to open a pull request or submit an issue if you find a bug or have a suggestion.

## License

This project is licensed under the MIT License.